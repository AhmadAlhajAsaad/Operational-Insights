use axum::{
    extract::DefaultBodyLimit,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use http::header;
use http::Method;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use equans_operational_insights_backend::{
    atlassian,
    auth::{self, JwtValidator},
    cache::CacheRepository,
    config::Config,
    github, github_cache, github_link, health, imports, jobs,
    organizations::OrganizationRepository,
    persons::PersonRepository,
    routes::{self, AppState, OrganizationState, PersonState},
    security,
};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file (if exists)
    dotenvy::dotenv().ok();

    // Logging / tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "equans_operational_insights_backend=debug,tower_http=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Configuration error: {}", e);
            tracing::error!("Make sure DATABASE_URL, ATLASSIAN_API_TOKEN, GITHUB_PAT_TOKEN, and GITHUB_ENTERPRISE_SLUG are set");
            std::process::exit(1);
        }
    };

    // Connect to database
    tracing::info!("Connecting to database...");
    let pool = match PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(600))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Database connected");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Run migrations
    tracing::info!("Running database migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => tracing::info!("Migrations completed"),
        Err(e) => {
            tracing::error!("Migration failed: {}", e);
            std::process::exit(1);
        }
    }

    // Initialize authentication (optional)
    let jwt_validator: Option<Arc<JwtValidator>> = config.auth.as_ref().map(|auth_config| {
        tracing::info!("Authentication enabled (Azure AD)");
        Arc::new(JwtValidator::new(&auth_config.to_auth_config()))
    });

    if jwt_validator.is_none() {
        tracing::warn!("Authentication DISABLED - API endpoints are unprotected!");
        tracing::warn!(
            "  Set AZURE_AD_TENANT_ID, AZURE_AD_CLIENT_ID, and AZURE_AD_AUDIENCE to enable"
        );
    }

    // Initialize services
    let atlassian_client = atlassian::AtlassianClient::new(config.atlassian_api_token.clone());
    let cache_repository = CacheRepository::new(pool.clone(), config.cache_ttl_hours);
    let atlassian_service = Arc::new(atlassian::AtlassianService::new(
        atlassian_client,
        cache_repository,
    ));

    // Initialize repositories for FR-005/006/007
    let person_repository = Arc::new(PersonRepository::new(pool.clone()));
    let org_repository = Arc::new(OrganizationRepository::new(pool.clone()));

    // Initialize import service for FR-007
    let import_service = Arc::new(imports::ImportService::new(pool.clone()));

    // Initialize Atlassian link service for FR-009
    let link_service = Arc::new(atlassian::AtlassianLinkService::new(pool.clone()));

    // Initialize GitHub API client (TR-011)  optional; warn if not configured
    let (github_pat, github_slug) = (
        config.github_pat_token.clone().unwrap_or_default(),
        config.github_enterprise_slug.clone().unwrap_or_default(),
    );
    if github_pat.is_empty() || github_slug.is_empty() {
        tracing::warn!(" GitHub integration DISABLED  set GITHUB_PAT_TOKEN and GITHUB_ENTERPRISE_SLUG to enable");
    } else {
        tracing::info!(
            " GitHub Enterprise client initialized (enterprise: {})",
            github_slug
        );
    }
    let github_client = std::sync::Arc::new(github::GitHubApiClient::new(github_pat, github_slug));

    // Initialize FR-012 GitHub cache and link services
    let github_cache_repo =
        std::sync::Arc::new(github_cache::GitHubCacheRepository::new(pool.clone()));
    let github_link_svc = std::sync::Arc::new(github_link::GitHubLinkService::new(pool.clone()));

    let github_state = github::GitHubState {
        client: github_client,
        pool: pool.clone(),
        cache_repo: github_cache_repo,
        link_service: github_link_svc,
    };
    // Run initial sync if cache is empty (non-blocking, just log errors)
    {
        let service = atlassian_service.clone();
        let link_svc = link_service.clone();
        tokio::spawn(async move {
            if let Err(e) = jobs::run_initial_sync_if_empty(&service, &link_svc).await {
                tracing::warn!("Initial sync skipped: {}", e);
            }
        });
    }

    // Start background sync job
    // Delay is set long to avoid competing with large imports for memory
    jobs::start_sync_job(
        atlassian_service.clone(),
        link_service.clone(),
        jobs::SyncJobConfig {
            initial_delay_secs: 3600,
            interval_hours: config.sync_interval_hours,
            org_id: None, // Will auto-detect
        },
    );

    // Start GitHub background sync job (FR-012 / TR-012)
    jobs::start_github_sync_job(
        github_state.client.clone(),
        github_state.cache_repo.clone(),
        github_state.link_service.clone(),
        jobs::GitHubSyncJobConfig {
            initial_delay_secs: 200, // 5-minute delay at startup
            interval_hours: config.sync_interval_hours,
        },
    );

    // Create app states
    let atlassian_state = AppState {
        atlassian_service,
        link_service: link_service.clone(),
    };
    let person_state = PersonState {
        repository: person_repository.clone(),
        link_service: Some(link_service.clone()),
    };
    let org_state = OrganizationState {
        org_repository: org_repository.clone(),
        person_repository: person_repository.clone(),
    };
    let import_state = routes::ImportState {
        service: import_service,
    };

    // CORS configuration - restricted per Privacy & Security Plan section 7
    let allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string());
    let origins: Vec<http::HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|o| o.trim().parse().ok())
        .collect();
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

    // Build the router based on auth configuration
    let app = build_router(
        atlassian_state,
        person_state,
        org_state,
        import_state,
        github_state,
        jwt_validator,
        cors,
    );

    // Read port from config
    let port = config.backend_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Try to bind to the port to check if it's available
    match std::net::TcpListener::bind(addr) {
        Ok(_) => {
            tracing::info!("Port {} is available", port);
        }
        Err(e) => {
            tracing::error!("Port {} is already in use!", port);
            tracing::error!("Error: {}", e);
            tracing::error!("");
            tracing::error!("To kill the existing process, run:");
            tracing::error!("  lsof -ti:{} | xargs kill -9", port);
            std::process::exit(1);
        }
    }

    tracing::info!("Starting backend on http://localhost:{}", port);
    tracing::info!("API endpoints available at http://localhost:{}/api", port);
    tracing::info!("  - GET /api/persons          (FR-005: Person list)",);
    tracing::info!("  - GET /api/organizations    (FR-006: Organization list)",);
    tracing::info!("  - POST /api/imports/upload  (FR-007: Import data)",);
    tracing::info!(
        "Cache TTL: {} hours, Sync interval: {} hours",
        config.cache_ttl_hours,
        config.sync_interval_hours
    );

    match axum_server::bind(addr).serve(app.into_make_service()).await {
        Ok(_) => {
            tracing::info!("Server shut down gracefully");
        }
        Err(e) => {
            tracing::error!("Server error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Build the application router
fn build_router(
    atlassian_state: AppState,
    person_state: PersonState,
    org_state: OrganizationState,
    import_state: routes::ImportState,
    github_state: github::GitHubState,
    jwt_validator: Option<Arc<JwtValidator>>,
    cors: CorsLayer,
) -> Router {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/", get(health::root));

    // Person routes (FR-005, FR-009)
    let person_routes = Router::new()
        .route("/", get(routes::persons::list_persons))
        .route("/stats", get(routes::persons::get_person_stats))
        .route("/inactive", get(routes::persons::get_inactive_persons))
        .route("/match-gids", post(routes::persons::match_gids))
        .route("/:person_id", get(routes::persons::get_person))
        .route("/", post(routes::persons::create_person))
        .route("/:person_id", put(routes::persons::update_person))
        // TS-06: GDPR right to erasure (Art. 17 AVG)
        .route("/:person_id", delete(routes::persons::delete_person))
        // FR-009: Atlassian data display (read-only)
        .route(
            "/:person_id/atlassian",
            get(routes::persons::get_person_atlassian_link),
        )
        .with_state(person_state.clone());

    // Organization routes (FR-006)
    let org_routes = Router::new()
        .route("/", get(routes::organizations::list_organizations))
        .route("/stats", get(routes::organizations::get_organization_stats))
        .route("/tree", get(routes::organizations::get_organization_tree))
        .route(
            "/billing-locations",
            get(routes::organizations::get_billing_location_distribution),
        )
        .route(
            "/business-units",
            get(routes::organizations::get_business_unit_distribution),
        )
        .route("/:org_id", get(routes::organizations::get_organization))
        .route("/", post(routes::organizations::create_organization))
        .route(
            "/:org_id/persons",
            get(routes::organizations::get_organization_persons),
        )
        .route(
            "/:org_id/atlassian-products",
            get(routes::organizations::get_organization_atlassian_products),
        )
        .route(
            "/:org_id/atlassian-linked-count",
            get(routes::organizations::get_organization_atlassian_linked_count),
        )
        .route(
            "/:org_id/github-products",
            get(routes::organizations::get_organization_github_products),
        )
        .route(
            "/global/atlassian-products",
            get(routes::organizations::get_global_atlassian_products),
        )
        .route(
            "/global/github-products",
            get(routes::organizations::get_global_github_products),
        )
        .route(
            "/global/linking-stats",
            get(routes::organizations::get_linking_stats),
        )
        .with_state(org_state);

    // Atlassian routes (existing + FR-008 User Management + FR-009 Link Management)
    let atlassian_routes = Router::new()
        .route("/organizations", get(routes::atlassian::get_organizations))
        .route(
            "/organizations/:org_id/users",
            get(routes::atlassian::get_users),
        )
        .route(
            "/organizations/:org_id/groups",
            get(routes::atlassian::get_groups),
        )
        .route(
            "/organizations/:org_id/licenses/:product",
            get(routes::atlassian::get_license_count),
        )
        .route(
            "/organizations/:org_id/licenses/:product/details",
            get(routes::atlassian::get_license_count_detailed),
        )
        // FR-008: User Management endpoints
        .route("/users", get(routes::atlassian::get_users_list))
        .route("/users", post(routes::atlassian::invite_user))
        .route("/users/sync", post(routes::atlassian::sync_users_manual))
        .route("/product-stats", get(routes::atlassian::get_product_stats))
        .route(
            "/sync-status/:sync_type",
            get(routes::atlassian::get_sync_status),
        )
        .route(
            "/users/:account_id",
            get(routes::atlassian::get_user_detail),
        )
        .route(
            "/users/:account_id/suspend",
            put(routes::atlassian::suspend_user),
        )
        .route("/users/:account_id", delete(routes::atlassian::delete_user))
        // FR-009: Person-to-Atlassian Linking
        .route(
            "/link-persons",
            post(routes::atlassian::link_persons_manual),
        )
        .with_state(atlassian_state.clone());

    // GitHub routes (FR-011 / TR-011 / FR-012 / TR-012)
    let github_routes = Router::new()
        .route("/validate", get(github::validate_token))
        .route("/overview", get(github::get_overview))
        .route("/copilot/seats", get(github::get_copilot_seats_list))
        .route("/ghas/users", get(github::get_ghas_users))
        .route("/license/users", get(github::get_license_users))
        // Backward-compatible endpoints
        .route(
            "/enterprises/:enterprise/licenses",
            get(github::get_licenses_compat),
        )
        .route(
            "/enterprises/:enterprise/copilot",
            get(github::get_copilot_seats_compat),
        )
        .route(
            "/enterprises/:enterprise/ghas",
            get(github::get_ghas_usage_compat),
        )
        // FR-012: person <-> GitHub link endpoints
        .route(
            "/persons/:person_id/github",
            get(github::get_person_github_link),
        )
        .route(
            "/persons/:person_id/github/link",
            post(github::link_person_github),
        )
        .route(
            "/persons/:person_id/github/link",
            delete(github::unlink_person_github),
        )
        .route(
            "/persons/:person_id/github/username",
            post(github::set_person_github_username),
        )
        // FR-012: organization <-> GitHub link endpoints
        .route(
            "/organizations/:org_id/github",
            get(github::get_org_github_info),
        )
        .route(
            "/organizations/:org_id/github",
            put(github::set_org_github_links),
        )
        // FR-012: admin sync endpoints
        .route("/admin/sync", post(github::trigger_github_sync))
        .route("/admin/sync/status", get(github::get_github_sync_status))
        .route("/admin/unlinked", get(github::get_unlinked_github_accounts))
        .with_state(github_state);

    // Import routes (FR-007)
    let import_routes = Router::new()
        .route("/upload", post(routes::imports::upload_file))
        .route("/preview", post(routes::imports::generate_preview))
        .route("/execute", post(routes::imports::execute_import))
        .route("/quick-import", post(routes::imports::quick_import))
        .route("/", get(routes::imports::list_imports))
        .route("/:import_id", get(routes::imports::get_import))
        .layer(DefaultBodyLimit::max(52_428_800)) // 50MB limit for file uploads
        .with_state(import_state);

    // Combined API routes
    let api_routes = Router::new()
        .nest("/persons", person_routes)
        .nest("/organizations", org_routes)
        .nest("/atlassian", atlassian_routes)
        .nest("/github", github_routes)
        .nest("/imports", import_routes)
        .fallback(api_fallback);

    // Build final router with optional authentication
    if let Some(validator) = jwt_validator {
        // Protected mode: API routes require authentication
        let protected_api = api_routes.layer(middleware::from_fn_with_state(
            validator.clone(),
            auth::auth_middleware,
        ));

        Router::new()
            .merge(public_routes)
            .nest("/api", protected_api)
            .layer(cors)
            .layer(middleware::from_fn(
                security::headers::security_headers_middleware,
            ))
            .layer(TraceLayer::new_for_http())
    } else {
        // Development mode: No authentication required
        Router::new()
            .merge(public_routes)
            .nest("/api", api_routes)
            .layer(cors)
            .layer(middleware::from_fn(
                security::headers::security_headers_middleware,
            ))
            .layer(TraceLayer::new_for_http())
    }
}

/// Catch-all handler for unknown API routes -- returns a structured JSON 404.
async fn api_fallback(uri: axum::http::Uri) -> axum::response::Response {
    let body = axum::Json(serde_json::json!({
        "error": "NOT_FOUND",
        "message": format!("No route matches {}", uri)
    }));
    (axum::http::StatusCode::NOT_FOUND, body).into_response()
}
