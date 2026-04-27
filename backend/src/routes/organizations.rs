//! Organization API route handlers (FR-006)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::error::AppError;
use crate::organizations::{
    CreateOrganizationRequest, OrganizationListParams, OrganizationRepository,
};
use crate::persons::{PaginatedResponse, PersonRepository, PersonSummary};

/// Shared state for organization routes
#[derive(Clone)]
pub struct OrganizationState {
    pub org_repository: Arc<OrganizationRepository>,
    pub person_repository: Arc<PersonRepository>,
}

/// GET /api/organizations - List all organizations
pub async fn list_organizations(
    State(state): State<OrganizationState>,
    Query(params): Query<OrganizationListParams>,
) -> Result<impl IntoResponse, AppError> {
    let (orgs, total) = state
        .org_repository
        .list(&params)
        .await
        .map_err(AppError::internal)?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(25);

    Ok(Json(PaginatedResponse::new(orgs, total, page, per_page)))
}

/// GET /api/organizations/:org_id - Get organization details
pub async fn get_organization(
    State(state): State<OrganizationState>,
    Path(org_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let org = state
        .org_repository
        .get_detail(&org_id)
        .await
        .map_err(AppError::internal)?;

    match org {
        Some(o) => Ok(Json(o)),
        None => Err(AppError::NotFound(format!(
            "Organization '{}' not found",
            org_id
        ))),
    }
}

/// POST /api/organizations - Create a new organization
pub async fn create_organization(
    State(state): State<OrganizationState>,
    Json(req): Json<CreateOrganizationRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check for duplicate
    if state
        .org_repository
        .get_by_id(&req.org_id)
        .await
        .map_err(AppError::internal)?
        .is_some()
    {
        return Err(AppError::Conflict(format!(
            "Organization '{}' already exists",
            req.org_id
        )));
    }

    let org = state
        .org_repository
        .create(&req)
        .await
        .map_err(AppError::internal)?;

    Ok((StatusCode::CREATED, Json(org)))
}

/// GET /api/organizations/:org_id/persons - Get persons in organization
pub async fn get_organization_persons(
    State(state): State<OrganizationState>,
    Path(org_id): Path<String>,
    Query(params): Query<OrgPersonsParams>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(25).min(100);

    let search = params.search.as_deref().filter(|s| s.len() >= 2);
    let atlassian_filter = params
        .atlassian_filter
        .as_deref()
        .filter(|s| matches!(*s, "linked" | "unlinked"));

    let (persons, total) = state
        .person_repository
        .get_by_org(&org_id, page, per_page, search, atlassian_filter)
        .await
        .map_err(AppError::internal)?;

    let summaries: Vec<PersonSummary> = persons.into_iter().map(PersonSummary::from).collect();
    Ok(Json(PaginatedResponse::new(
        summaries, total, page, per_page,
    )))
}

/// GET /api/organizations/:org_id/atlassian-linked-count - Org-wide Atlassian linked count
pub async fn get_organization_atlassian_linked_count(
    State(state): State<OrganizationState>,
    Path(org_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (linked, total) = state
        .person_repository
        .get_atlassian_linked_count(&org_id)
        .await
        .map_err(AppError::internal)?;

    Ok(Json(
        serde_json::json!({ "linked": linked, "total": total }),
    ))
}

/// GET /api/organizations/tree - Get organization tree structure
pub async fn get_organization_tree(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let tree = state
        .org_repository
        .get_tree()
        .await
        .map_err(AppError::internal)?;

    Ok(Json(tree))
}

/// GET /api/organizations/stats - Get organization statistics
pub async fn get_organization_stats(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let stats = state
        .org_repository
        .get_stats()
        .await
        .map_err(AppError::internal)?;

    Ok(Json(stats))
}

/// GET /api/organizations/:org_id/atlassian-products - Per-product user counts
pub async fn get_organization_atlassian_products(
    State(state): State<OrganizationState>,
    Path(org_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let rows = state
        .org_repository
        .get_atlassian_product_counts(&org_id)
        .await
        .map_err(AppError::internal)?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(product_key, user_count)| {
            serde_json::json!({
                "product_key": product_key,
                "user_count": user_count,
            })
        })
        .collect();

    Ok(Json(result))
}

/// GET /api/organizations/:org_id/github-products - Per-product GitHub user counts
pub async fn get_organization_github_products(
    State(state): State<OrganizationState>,
    Path(org_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let rows = state
        .org_repository
        .get_github_product_counts(&org_id)
        .await
        .map_err(AppError::internal)?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(product_key, user_count)| {
            serde_json::json!({
                "product_key": product_key,
                "user_count": user_count,
            })
        })
        .collect();

    Ok(Json(result))
}

/// GET /api/organizations/billing-locations - Global billing location distribution
pub async fn get_billing_location_distribution(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let distribution = state
        .org_repository
        .get_global_billing_location_distribution()
        .await
        .map_err(AppError::internal)?;

    Ok(Json(distribution))
}

/// GET /api/organizations/business-units - Business unit (org_type) distribution
pub async fn get_business_unit_distribution(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let distribution = state
        .org_repository
        .get_business_unit_distribution()
        .await
        .map_err(AppError::internal)?;

    Ok(Json(distribution))
}

#[derive(Debug, serde::Deserialize)]
pub struct OrgPersonsParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub atlassian_filter: Option<String>,
}

/// GET /api/organizations/global/atlassian-products - Global Atlassian product counts (not org-scoped)
pub async fn get_global_atlassian_products(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let rows = state
        .org_repository
        .get_global_atlassian_product_counts()
        .await
        .map_err(AppError::internal)?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(product_key, user_count)| {
            serde_json::json!({
                "product_key": product_key,
                "user_count": user_count,
            })
        })
        .collect();

    Ok(Json(result))
}

/// GET /api/organizations/global/github-products - Global GitHub product counts
pub async fn get_global_github_products(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let rows = state
        .org_repository
        .get_global_github_product_counts()
        .await
        .map_err(AppError::internal)?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(product_key, user_count)| {
            serde_json::json!({
                "product_key": product_key,
                "user_count": user_count,
            })
        })
        .collect();

    Ok(Json(result))
}

/// GET /api/organizations/linking-stats - Data quality / linking statistics
pub async fn get_linking_stats(
    State(state): State<OrganizationState>,
) -> Result<impl IntoResponse, AppError> {
    let stats = state
        .org_repository
        .get_linking_stats()
        .await
        .map_err(AppError::internal)?;

    Ok(Json(stats))
}
