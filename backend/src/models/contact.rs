use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactStatus {
    Lead,
    Customer,
    Partner,
    Investor,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Option<Thing>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub tags: Vec<String>,
    pub status: ContactStatus,
    pub engagement_score: f64,
    pub company: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContactRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<ContactStatus>,
    pub company_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContactRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<ContactStatus>,
    pub engagement_score: Option<f64>,
    pub company_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactQuery {
    pub search: Option<String>,
    pub status: Option<ContactStatus>,
    pub tags: Option<String>,
    pub company_id: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub tags: Vec<String>,
    pub status: ContactStatus,
    pub engagement_score: f64,
    pub company_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Contact> for ContactResponse {
    fn from(c: Contact) -> Self {
        Self {
            id: c.id.map(|t| t.id.to_string()).unwrap_or_default(),
            first_name: c.first_name,
            last_name: c.last_name,
            email: c.email,
            phone: c.phone,
            linkedin_url: c.linkedin_url,
            tags: c.tags,
            status: c.status,
            engagement_score: c.engagement_score,
            company_id: c.company.map(|t| t.id.to_string()),
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

impl ContactResponse {
    /// Create a ContactResponse from a StoredContact (domain + ID)
    pub fn from_stored(stored: crate::repositories::StoredContact) -> Self {
        use crate::domain::ContactStatus as DomainStatus;

        // Convert domain status to API status
        let status = match stored.contact.status {
            DomainStatus::Lead => ContactStatus::Lead,
            DomainStatus::Customer => ContactStatus::Customer,
            DomainStatus::Partner => ContactStatus::Partner,
            DomainStatus::Investor => ContactStatus::Investor,
            DomainStatus::Other => ContactStatus::Other,
        };

        Self {
            id: stored.id,
            first_name: stored.contact.first_name,
            last_name: stored.contact.last_name,
            email: stored.contact.email,
            phone: stored.contact.phone,
            linkedin_url: stored.contact.linkedin_url,
            tags: stored.contact.tags,
            status,
            engagement_score: stored.contact.engagement_score,
            company_id: stored.contact.company_id,
            created_at: stored.contact.created_at,
            updated_at: stored.contact.updated_at,
        }
    }
}
