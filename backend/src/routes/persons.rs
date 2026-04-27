//! Person API route handlers (FR-005)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::atlassian::link_service::AtlassianLinkService;
use crate::error::AppError;
use crate::persons::{
    CreatePersonRequest, GidMatcher, PaginatedResponse, PersonDetail, PersonListParams,
    PersonRepository, PersonSummary,
};

/// Shared state for person routes
#[derive(Clone)]
pub struct PersonState {
    pub repository: Arc<PersonRepository>,
    pub link_service: Option<Arc<AtlassianLinkService>>,
}

/// GET /api/persons - List all persons with pagination and filtering
pub async fn list_persons(
    State(state): State<PersonState>,
    Query(params): Query<PersonListParams>,
) -> Result<impl IntoResponse, AppError> {
    let (persons, total) = state
        .repository
        .list(&params)
        .await
        .map_err(AppError::internal)?;

    let summaries: Vec<PersonSummary> = persons.into_iter().map(PersonSummary::from).collect();
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(25);

    Ok(Json(PaginatedResponse::new(
        summaries, total, page, per_page,
    )))
}

/// GET /api/persons/:person_id - Get person details
pub async fn get_person(
    State(state): State<PersonState>,
    Path(person_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let person_detail = state
        .repository
        .get_by_id_with_atlassian(&person_id)
        .await
        .map_err(AppError::internal)?;

    match person_detail {
        Some(detail) => Ok(Json(detail)),
        None => Err(AppError::NotFound(format!(
            "Person '{}' not found",
            person_id
        ))),
    }
}

/// POST /api/persons - Create a new person
pub async fn create_person(
    State(state): State<PersonState>,
    Json(req): Json<CreatePersonRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check for duplicate
    if state
        .repository
        .get_by_id(&req.person_id)
        .await
        .map_err(AppError::internal)?
        .is_some()
    {
        return Err(AppError::Conflict(format!(
            "Person '{}' already exists",
            req.person_id
        )));
    }

    let person = state
        .repository
        .create(&req)
        .await
        .map_err(AppError::internal)?;

    Ok((StatusCode::CREATED, Json(PersonDetail::from(person))))
}

/// PUT /api/persons/:person_id - Update a person
pub async fn update_person(
    State(state): State<PersonState>,
    Path(person_id): Path<String>,
    Json(req): Json<CreatePersonRequest>,
) -> Result<impl IntoResponse, AppError> {
    let person = state
        .repository
        .update(&person_id, &req)
        .await
        .map_err(AppError::internal)?;

    match person {
        Some(p) => Ok(Json(PersonDetail::from(p))),
        None => Err(AppError::NotFound(format!(
            "Person '{}' not found",
            person_id
        ))),
    }
}

/// GET /api/persons/inactive - Get inactive persons
pub async fn get_inactive_persons(
    State(state): State<PersonState>,
) -> Result<impl IntoResponse, AppError> {
    let persons = state
        .repository
        .get_inactive()
        .await
        .map_err(AppError::internal)?;

    let summaries: Vec<PersonSummary> = persons.into_iter().map(PersonSummary::from).collect();
    Ok(Json(summaries))
}

/// GET /api/persons/stats - Get person statistics
pub async fn get_person_stats(
    State(state): State<PersonState>,
) -> Result<impl IntoResponse, AppError> {
    let stats = state
        .repository
        .get_stats()
        .await
        .map_err(AppError::internal)?;

    Ok(Json(stats))
}

/// POST /api/persons/match-gids - Batch match GIDs for all persons
pub async fn match_gids(State(state): State<PersonState>) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Starting batch GID matching for all persons");

    let matcher = GidMatcher::new();

    const BATCH_SIZE: i64 = 1000;
    let mut offset = 0i64;
    let mut total_processed = 0u64;
    let mut total_matched = 0u64;

    loop {
        // Fetch batch of persons
        let persons = state
            .repository
            .get_all_for_gid_matching(BATCH_SIZE, offset)
            .await
            .map_err(AppError::internal)?;

        if persons.is_empty() {
            break;
        }

        let batch_size = persons.len();
        tracing::info!(
            "Processing batch at offset {}: {} persons",
            offset,
            batch_size
        );

        // Match GIDs for this batch
        let matches = matcher.match_batch(&persons);

        // Prepare updates (only for successful matches)
        let updates: Vec<(String, String, i32, String)> = matches
            .into_iter()
            .filter_map(|(person_id, match_opt)| {
                match_opt.map(|m| (person_id, m.gid, m.confidence, m.extraction_method))
            })
            .collect();

        let matched_count = updates.len();
        total_matched += matched_count as u64;

        // Batch update database
        if !updates.is_empty() {
            state
                .repository
                .batch_update_gid_matches(&updates)
                .await
                .map_err(AppError::internal)?;

            tracing::info!("Updated {} persons with GID matches", matched_count);
        }

        total_processed += batch_size as u64;
        offset += BATCH_SIZE;

        // Log progress every 10 batches
        if offset % (BATCH_SIZE * 10) == 0 {
            tracing::info!(
                "Progress: {} persons processed, {} matched",
                total_processed,
                total_matched
            );
        }
    }

    tracing::info!(
        "GID matching complete: {} persons processed, {} matched",
        total_processed,
        total_matched
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "total_processed": total_processed,
        "total_matched": total_matched,
        "match_rate": if total_processed > 0 {
            total_matched as f64 / total_processed as f64 * 100.0
        } else {
            0.0
        }
    })))
}

// ============================================================================
// Atlassian Data Display (FR-009 - Read-Only)
// ============================================================================

/// GET /api/persons/:person_id/atlassian - Get person's Atlassian link status and data
pub async fn get_person_atlassian_link(
    State(state): State<PersonState>,
    Path(person_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let link_service = state.link_service.as_ref().ok_or(AppError::Internal(
        "Atlassian link service not available".to_string(),
    ))?;

    let link_info = link_service
        .get_person_atlassian_link(&person_id)
        .await
        .map_err(|e| match e {
            crate::atlassian::ServiceError::NotFound(msg) => AppError::NotFound(msg),
            _ => AppError::internal(e),
        })?;

    Ok(Json(link_info))
}

// ============================================================================
// GDPR Right to Erasure (TS-06, Art. 17 AVG)
// ============================================================================

/// DELETE /api/persons/:person_id - Permanently delete a person and all related PII
///
/// Implements GDPR Art. 17 (right to erasure). Removes the person record
/// and all related data from the database. This action is irreversible.
pub async fn delete_person(
    State(state): State<PersonState>,
    Path(person_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Verify person exists
    let person = state
        .repository
        .get_by_id(&person_id)
        .await
        .map_err(AppError::internal)?;

    if person.is_none() {
        return Err(AppError::NotFound(format!(
            "Person '{}' not found",
            person_id
        )));
    }

    // Log the deletion for audit trail (with masked PII)
    let masked_email = person
        .as_ref()
        .map(|p| crate::security::masking::mask_email(&p.email))
        .unwrap_or_default();
    tracing::info!(
        person_id = %person_id,
        masked_email = %masked_email,
        "GDPR deletion request: removing person and all related PII (Art. 17 AVG)"
    );

    // Perform hard delete of person and all related data
    let deleted = state
        .repository
        .delete_person(&person_id)
        .await
        .map_err(AppError::internal)?;

    if deleted {
        Ok((StatusCode::NO_CONTENT, ()))
    } else {
        Err(AppError::Internal("Failed to delete person".to_string()))
    }
}
