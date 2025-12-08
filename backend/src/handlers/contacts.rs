//! Contact Handlers - HTTP endpoints for contact operations
//!
//! These handlers are thin - they:
//! 1. Extract request data from HTTP
//! 2. Call the ContactService
//! 3. Transform results to HTTP responses
//!
//! Business logic lives in the service and domain layers.

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::domain::ContactStatus as DomainStatus;
use crate::error::AppResult;
use crate::models::{ContactQuery, ContactResponse, CreateContactRequest, UpdateContactRequest};
use crate::repositories::ContactQuery as RepoContactQuery;
use crate::services::{CreateContactInput, UpdateContactInput};
use crate::AppState;

/// List contacts with optional filters
///
/// GET /api/contacts?limit=50&offset=0&status=lead&search=john
pub async fn list_contacts(
    State(state): State<AppState>,
    Query(query): Query<ContactQuery>,
) -> AppResult<Json<Vec<ContactResponse>>> {
    // Convert API query params to repository query
    let repo_query = RepoContactQuery::new()
        .with_limit(query.limit.unwrap_or(50))
        .with_offset(query.offset.unwrap_or(0));

    let contacts = state.contact_service.list(repo_query).await?;

    let responses: Vec<ContactResponse> = contacts
        .into_iter()
        .map(|stored| ContactResponse::from_stored(stored))
        .collect();

    Ok(Json(responses))
}

/// Create a new contact
///
/// POST /api/contacts
/// Body: { first_name, last_name, email, phone?, linkedin_url?, tags?, status?, company_id? }
pub async fn create_contact(
    State(state): State<AppState>,
    Json(req): Json<CreateContactRequest>,
) -> AppResult<Json<ContactResponse>> {
    let input = CreateContactInput {
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email,
        phone: req.phone,
        linkedin_url: req.linkedin_url,
        tags: req.tags.unwrap_or_default(),
        status: req.status.map(|s| api_status_to_domain(s)),
        company_id: req.company_id,
    };

    let stored = state.contact_service.create(input).await?;

    Ok(Json(ContactResponse::from_stored(stored)))
}

/// Get a single contact by ID
///
/// GET /api/contacts/:id
pub async fn get_contact(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ContactResponse>> {
    let stored = state.contact_service.get(&id).await?;

    Ok(Json(ContactResponse::from_stored(stored)))
}

/// Update an existing contact
///
/// PATCH /api/contacts/:id
/// Body: { first_name?, last_name?, email?, phone?, linkedin_url?, tags?, status?, engagement_score?, company_id? }
pub async fn update_contact(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateContactRequest>,
) -> AppResult<Json<ContactResponse>> {
    let input = UpdateContactInput {
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email,
        phone: req.phone,
        linkedin_url: req.linkedin_url,
        tags: req.tags,
        status: req.status.map(|s| api_status_to_domain(s)),
        engagement_score: req.engagement_score,
        company_id: req.company_id,
    };

    let stored = state.contact_service.update(&id, input).await?;

    Ok(Json(ContactResponse::from_stored(stored)))
}

/// Delete a contact
///
/// DELETE /api/contacts/:id
pub async fn delete_contact(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    state.contact_service.delete(&id).await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}

// Helper function to convert API status to domain status
fn api_status_to_domain(status: crate::models::ContactStatus) -> DomainStatus {
    match status {
        crate::models::ContactStatus::Lead => DomainStatus::Lead,
        crate::models::ContactStatus::Customer => DomainStatus::Customer,
        crate::models::ContactStatus::Partner => DomainStatus::Partner,
        crate::models::ContactStatus::Investor => DomainStatus::Investor,
        crate::models::ContactStatus::Other => DomainStatus::Other,
    }
}
