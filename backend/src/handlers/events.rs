use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use surrealdb::sql::Thing;

use crate::error::{AppError, AppResult};
use crate::models::{
    CreateEventRequest, Event, EventResponse, InviteRequest, Rsvp, RsvpRequest, RsvpResponse,
    RsvpStatus, TimelineEntry, TimelineEntryType,
};
use crate::AppState;

pub async fn list_events(State(state): State<AppState>) -> AppResult<Json<Vec<EventResponse>>> {
    let events: Vec<Event> = state
        .db
        .client
        .query("SELECT * FROM event ORDER BY start_time ASC")
        .await?
        .take(0)?;

    let responses: Vec<EventResponse> = events.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_event(
    State(state): State<AppState>,
    Json(req): Json<CreateEventRequest>,
) -> AppResult<Json<EventResponse>> {
    let campaign = req.campaign_id.map(|id| Thing::from(("campaign", id.as_str())));

    let events: Vec<Event> = state
        .db
        .client
        .create("event")
        .content(Event {
            id: None,
            campaign,
            name: req.name,
            event_type: req.event_type,
            description: req.description,
            start_time: req.start_time,
            end_time: req.end_time,
            location: req.location,
            created_at: Utc::now(),
        })
        .await?;

    let event = events.into_iter().next().ok_or_else(|| AppError::Internal("Failed to create event".into()))?;
    Ok(Json(event.into()))
}

pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<EventResponse>> {
    let event: Option<Event> = state
        .db
        .client
        .select(("event", id.as_str()))
        .await?;

    let event = event.ok_or_else(|| AppError::NotFound("Event not found".into()))?;
    Ok(Json(event.into()))
}

pub async fn invite_to_event(
    State(state): State<AppState>,
    Path(event_id): Path<String>,
    Json(req): Json<InviteRequest>,
) -> AppResult<Json<Vec<RsvpResponse>>> {
    let event_thing = Thing::from(("event", event_id.as_str()));
    let mut rsvps = Vec::new();

    for contact_id in req.contact_ids {
        let contact_thing = Thing::from(("contact", contact_id.as_str()));

        // Create RSVP with invited status
        let rsvp_results: Vec<Rsvp> = state
            .db
            .client
            .create("rsvp")
            .content(Rsvp {
                id: None,
                event: event_thing.clone(),
                contact: contact_thing.clone(),
                status: RsvpStatus::Invited,
                timestamp: Utc::now(),
            })
            .await?;

        if let Some(r) = rsvp_results.into_iter().next() {
            rsvps.push(r.into());
        }

        // Create timeline entry
        let _: Vec<TimelineEntry> = state
            .db
            .client
            .create("timeline_entry")
            .content(TimelineEntry {
                id: None,
                contact: contact_thing,
                company: None,
                entry_type: TimelineEntryType::EventInvite,
                content: format!("Invited to event {}", event_id),
                metadata: serde_json::json!({ "event_id": event_id }),
                timestamp: Utc::now(),
            })
            .await?;
    }

    Ok(Json(rsvps))
}

pub async fn rsvp_event(
    State(state): State<AppState>,
    Path(event_id): Path<String>,
    Json(req): Json<RsvpRequest>,
) -> AppResult<Json<RsvpResponse>> {
    let event_thing = Thing::from(("event", event_id.as_str()));
    let contact_thing = Thing::from(("contact", req.contact_id.as_str()));

    // Find existing RSVP
    let existing: Vec<Rsvp> = state
        .db
        .client
        .query("SELECT * FROM rsvp WHERE event = $event AND contact = $contact LIMIT 1")
        .bind(("event", event_thing.clone()))
        .bind(("contact", contact_thing.clone()))
        .await?
        .take(0)?;

    let rsvp = if let Some(existing_rsvp) = existing.first() {
        // Update existing RSVP
        let updated: Option<Rsvp> = state
            .db
            .client
            .update(existing_rsvp.id.clone().unwrap())
            .merge(serde_json::json!({
                "status": req.status,
                "timestamp": Utc::now()
            }))
            .await?;

        updated.ok_or_else(|| AppError::Internal("Failed to update RSVP".into()))?
    } else {
        // Create new RSVP
        let new_rsvps: Vec<Rsvp> = state
            .db
            .client
            .create("rsvp")
            .content(Rsvp {
                id: None,
                event: event_thing,
                contact: contact_thing.clone(),
                status: req.status.clone(),
                timestamp: Utc::now(),
            })
            .await?;

        new_rsvps.into_iter().next().ok_or_else(|| AppError::Internal("Failed to create RSVP".into()))?
    };

    // Create timeline entry if registered or attended
    if matches!(req.status, RsvpStatus::Registered | RsvpStatus::Attended) {
        let entry_type = match req.status {
            RsvpStatus::Attended => TimelineEntryType::EventAttend,
            _ => TimelineEntryType::EventInvite,
        };

        let _: Vec<TimelineEntry> = state
            .db
            .client
            .create("timeline_entry")
            .content(TimelineEntry {
                id: None,
                contact: contact_thing,
                company: None,
                entry_type,
                content: format!("RSVP status updated for event {}", event_id),
                metadata: serde_json::json!({
                    "event_id": event_id,
                    "status": req.status
                }),
                timestamp: Utc::now(),
            })
            .await?;
    }

    Ok(Json(rsvp.into()))
}
