use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Webinar,
    Meetup,
    Ama,
    Demo,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<Thing>,
    pub campaign: Option<Thing>,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RsvpStatus {
    Invited,
    Registered,
    Attended,
    NoShow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rsvp {
    pub id: Option<Thing>,
    pub event: Thing,
    pub contact: Thing,
    pub status: RsvpStatus,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub campaign_id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: String,
}

#[derive(Debug, Deserialize)]
pub struct InviteRequest {
    pub contact_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RsvpRequest {
    pub contact_id: String,
    pub status: RsvpStatus,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub id: String,
    pub campaign_id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: String,
    pub created_at: DateTime<Utc>,
}

impl From<Event> for EventResponse {
    fn from(e: Event) -> Self {
        Self {
            id: e.id.map(|t| t.id.to_string()).unwrap_or_default(),
            campaign_id: e.campaign.map(|t| t.id.to_string()),
            name: e.name,
            event_type: e.event_type,
            description: e.description,
            start_time: e.start_time,
            end_time: e.end_time,
            location: e.location,
            created_at: e.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RsvpResponse {
    pub id: String,
    pub event_id: String,
    pub contact_id: String,
    pub status: RsvpStatus,
    pub timestamp: DateTime<Utc>,
}

impl From<Rsvp> for RsvpResponse {
    fn from(r: Rsvp) -> Self {
        Self {
            id: r.id.map(|t| t.id.to_string()).unwrap_or_default(),
            event_id: r.event.id.to_string(),
            contact_id: r.contact.id.to_string(),
            status: r.status,
            timestamp: r.timestamp,
        }
    }
}
