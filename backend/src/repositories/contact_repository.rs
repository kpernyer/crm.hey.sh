//! Contact Repository - Database operations for contacts
//!
//! This layer handles:
//! - CRUD operations against SurrealDB
//! - Mapping between domain::Contact and database records
//! - Query building for filters/search
//! - Handling database-level constraints (unique email)

use crate::db::Database;
use crate::domain::{Contact as DomainContact, ContactStatus as DomainStatus};
use crate::error::{AppError, AppResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;

/// Database representation of a Contact
/// This is what SurrealDB stores/returns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactRecord {
    pub id: Option<Thing>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub tags: Vec<String>,
    pub status: String, // Stored as string in DB
    pub engagement_score: f64,
    pub company: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Query parameters for listing contacts
#[derive(Debug, Default)]
pub struct ContactQuery {
    pub search: Option<String>,
    pub status: Option<DomainStatus>,
    pub tags: Option<Vec<String>>,
    pub company_id: Option<String>,
    pub min_engagement: Option<f64>,
    pub max_engagement: Option<f64>,
    pub limit: u32,
    pub offset: u32,
}

impl ContactQuery {
    pub fn new() -> Self {
        Self {
            limit: 50,
            offset: 0,
            ..Default::default()
        }
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_status(mut self, status: DomainStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }
}

/// Repository for Contact database operations
pub struct ContactRepository {
    db: Arc<Database>,
}

impl ContactRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Find a contact by ID
    pub async fn find_by_id(&self, id: &str) -> AppResult<Option<DomainContact>> {
        let record: Option<ContactRecord> = self
            .db
            .client
            .select(("contact", id))
            .await?;

        Ok(record.map(|r| self.to_domain(r)))
    }

    /// Find a contact by email (for uniqueness checks)
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<DomainContact>> {
        let records: Vec<ContactRecord> = self
            .db
            .client
            .query("SELECT * FROM contact WHERE email = $email LIMIT 1")
            .bind(("email", email.to_lowercase()))
            .await?
            .take(0)?;

        Ok(records.into_iter().next().map(|r| self.to_domain(r)))
    }

    /// Check if email exists (excluding a specific contact ID)
    pub async fn email_exists_for_other(&self, email: &str, exclude_id: &str) -> AppResult<bool> {
        let records: Vec<ContactRecord> = self
            .db
            .client
            .query("SELECT * FROM contact WHERE email = $email AND id != $id LIMIT 1")
            .bind(("email", email.to_lowercase()))
            .bind(("id", Thing::from(("contact", exclude_id))))
            .await?
            .take(0)?;

        Ok(!records.is_empty())
    }

    /// List contacts with optional filters
    pub async fn find_all(&self, query: ContactQuery) -> AppResult<Vec<DomainContact>> {
        let mut conditions = Vec::new();
        let mut bindings: Vec<(&str, serde_json::Value)> = Vec::new();

        // Build WHERE conditions dynamically
        if let Some(ref status) = query.status {
            conditions.push("status = $status");
            bindings.push(("status", serde_json::json!(status_to_string(status))));
        }

        if let Some(ref search) = query.search {
            conditions.push("(first_name CONTAINS $search OR last_name CONTAINS $search OR email CONTAINS $search)");
            bindings.push(("search", serde_json::json!(search)));
        }

        if let Some(min) = query.min_engagement {
            conditions.push("engagement_score >= $min_engagement");
            bindings.push(("min_engagement", serde_json::json!(min)));
        }

        if let Some(max) = query.max_engagement {
            conditions.push("engagement_score <= $max_engagement");
            bindings.push(("max_engagement", serde_json::json!(max)));
        }

        if let Some(ref company_id) = query.company_id {
            conditions.push("company = $company");
            bindings.push(("company", serde_json::json!(format!("company:{}", company_id))));
        }

        // Build query string
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query_str = format!(
            "SELECT * FROM contact {} ORDER BY created_at DESC LIMIT $limit START $offset",
            where_clause
        );

        let mut db_query = self.db.client.query(&query_str);

        // Bind all parameters
        for (key, value) in bindings {
            db_query = db_query.bind((key, value));
        }

        db_query = db_query
            .bind(("limit", query.limit))
            .bind(("offset", query.offset));

        let records: Vec<ContactRecord> = db_query.await?.take(0)?;

        Ok(records.into_iter().map(|r| self.to_domain(r)).collect())
    }

    /// Create a new contact
    pub async fn create(&self, contact: &DomainContact) -> AppResult<DomainContact> {
        let record = self.to_record(contact);

        let created: Vec<ContactRecord> = self
            .db
            .client
            .create("contact")
            .content(record)
            .await?;

        let created = created
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to create contact".into()))?;

        Ok(self.to_domain(created))
    }

    /// Update an existing contact
    pub async fn update(&self, id: &str, contact: &DomainContact) -> AppResult<DomainContact> {
        let record = self.to_record(contact);

        let updated: Option<ContactRecord> = self
            .db
            .client
            .update(("contact", id))
            .content(record)
            .await?;

        let updated = updated
            .ok_or_else(|| AppError::NotFound(format!("Contact {} not found", id)))?;

        Ok(self.to_domain(updated))
    }

    /// Delete a contact
    pub async fn delete(&self, id: &str) -> AppResult<bool> {
        let _: Option<ContactRecord> = self
            .db
            .client
            .delete(("contact", id))
            .await?;

        Ok(true)
    }

    /// Count contacts matching a query
    pub async fn count(&self, query: ContactQuery) -> AppResult<u64> {
        // Simplified - in production, reuse query building logic
        let records: Vec<ContactRecord> = self
            .db
            .client
            .query("SELECT count() FROM contact GROUP ALL")
            .await?
            .take(0)?;

        // SurrealDB returns count in a specific format
        Ok(records.len() as u64)
    }

    // ---- Mapping Functions ----

    /// Convert database record to domain model
    fn to_domain(&self, record: ContactRecord) -> DomainContact {
        DomainContact {
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            phone: record.phone,
            linkedin_url: record.linkedin_url,
            tags: record.tags,
            status: string_to_status(&record.status),
            engagement_score: record.engagement_score,
            company_id: record.company.map(|t| t.id.to_string()),
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }

    /// Convert domain model to database record
    fn to_record(&self, contact: &DomainContact) -> ContactRecord {
        ContactRecord {
            id: None, // Let DB generate
            first_name: contact.first_name.clone(),
            last_name: contact.last_name.clone(),
            email: contact.email.clone(),
            phone: contact.phone.clone(),
            linkedin_url: contact.linkedin_url.clone(),
            tags: contact.tags.clone(),
            status: status_to_string(&contact.status),
            engagement_score: contact.engagement_score,
            company: contact.company_id.as_ref().map(|id| Thing::from(("company", id.as_str()))),
            created_at: contact.created_at,
            updated_at: contact.updated_at,
        }
    }
}

// ---- Helper Functions ----

fn status_to_string(status: &DomainStatus) -> String {
    match status {
        DomainStatus::Lead => "lead".to_string(),
        DomainStatus::Customer => "customer".to_string(),
        DomainStatus::Partner => "partner".to_string(),
        DomainStatus::Investor => "investor".to_string(),
        DomainStatus::Other => "other".to_string(),
    }
}

fn string_to_status(s: &str) -> DomainStatus {
    match s {
        "lead" => DomainStatus::Lead,
        "customer" => DomainStatus::Customer,
        "partner" => DomainStatus::Partner,
        "investor" => DomainStatus::Investor,
        _ => DomainStatus::Other,
    }
}

/// Stored contact with its ID (for API responses)
#[derive(Debug, Clone)]
pub struct StoredContact {
    pub id: String,
    pub contact: DomainContact,
}

impl ContactRepository {
    /// Find by ID and return with ID attached
    pub async fn find_by_id_with_id(&self, id: &str) -> AppResult<Option<StoredContact>> {
        let record: Option<ContactRecord> = self
            .db
            .client
            .select(("contact", id))
            .await?;

        Ok(record.map(|r| StoredContact {
            id: r.id.as_ref().map(|t| t.id.to_string()).unwrap_or_default(),
            contact: self.to_domain(r),
        }))
    }

    /// Create and return with ID
    pub async fn create_with_id(&self, contact: &DomainContact) -> AppResult<StoredContact> {
        let record = self.to_record(contact);

        let created: Vec<ContactRecord> = self
            .db
            .client
            .create("contact")
            .content(record)
            .await?;

        let created = created
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to create contact".into()))?;

        Ok(StoredContact {
            id: created.id.as_ref().map(|t| t.id.to_string()).unwrap_or_default(),
            contact: self.to_domain(created),
        })
    }
}
