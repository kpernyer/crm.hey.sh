use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineEntryType {
    EmailSent,
    EmailOpen,
    EmailClick,
    SocialTouch,
    Note,
    EventInvite,
    EventAttend,
    LandingPageVisit,
    Task,
    Call,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub id: Option<Thing>,
    pub contact: Thing,
    pub company: Option<Thing>,
    #[serde(rename = "type")]
    pub entry_type: TimelineEntryType,
    pub content: String,
    pub metadata: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTimelineEntryRequest {
    pub contact_id: String,
    pub company_id: Option<String>,
    #[serde(rename = "type")]
    pub entry_type: TimelineEntryType,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineQuery {
    pub contact_id: Option<String>,
    pub company_id: Option<String>,
    pub entry_type: Option<TimelineEntryType>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TimelineEntryResponse {
    pub id: String,
    pub contact_id: String,
    pub company_id: Option<String>,
    #[serde(rename = "type")]
    pub entry_type: TimelineEntryType,
    pub content: String,
    pub metadata: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

impl From<TimelineEntry> for TimelineEntryResponse {
    fn from(t: TimelineEntry) -> Self {
        Self {
            id: t.id.map(|th| th.id.to_string()).unwrap_or_default(),
            contact_id: t.contact.id.to_string(),
            company_id: t.company.map(|th| th.id.to_string()),
            entry_type: t.entry_type,
            content: t.content,
            metadata: t.metadata,
            timestamp: t.timestamp,
        }
    }
}
