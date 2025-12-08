use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use surrealdb::sql::Thing;

use crate::error::{AppError, AppResult};
use crate::models::{
    CreateTimelineEntryRequest, TimelineEntry, TimelineEntryResponse, TimelineQuery,
};
use crate::AppState;

pub async fn get_contact_timeline(
    State(state): State<AppState>,
    Path(contact_id): Path<String>,
    Query(query): Query<TimelineQuery>,
) -> AppResult<Json<Vec<TimelineEntryResponse>>> {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let entries: Vec<TimelineEntry> = state
        .db
        .client
        .query("SELECT * FROM timeline_entry WHERE contact = $contact ORDER BY timestamp DESC LIMIT $limit START $offset")
        .bind(("contact", Thing::from(("contact", contact_id.as_str()))))
        .bind(("limit", limit))
        .bind(("offset", offset))
        .await?
        .take(0)?;

    let responses: Vec<TimelineEntryResponse> = entries.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_timeline_entry(
    State(state): State<AppState>,
    Json(req): Json<CreateTimelineEntryRequest>,
) -> AppResult<Json<TimelineEntryResponse>> {
    let contact = Thing::from(("contact", req.contact_id.as_str()));
    let company = req.company_id.map(|id| Thing::from(("company", id.as_str())));

    let entries: Vec<TimelineEntry> = state
        .db
        .client
        .create("timeline_entry")
        .content(TimelineEntry {
            id: None,
            contact,
            company,
            entry_type: req.entry_type,
            content: req.content,
            metadata: req.metadata.unwrap_or(serde_json::json!({})),
            timestamp: Utc::now(),
        })
        .await?;

    let entry = entries.into_iter().next().ok_or_else(|| AppError::Internal("Failed to create timeline entry".into()))?;
    Ok(Json(entry.into()))
}
