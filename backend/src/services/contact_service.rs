//! Contact Service - Orchestrates domain logic and repository operations
//!
//! The service layer is the "glue" between:
//! - Handlers (HTTP layer) - receive requests
//! - Domain (business logic) - validate and transform
//! - Repository (data access) - persist to database
//!
//! The service enforces business rules that require database access,
//! like "email must be unique".

use std::sync::Arc;

use crate::db::Database;
use crate::domain::{Contact, ContactBuilder, ContactStatus, ContactUpdater};
use crate::error::{AppError, AppResult};
use crate::repositories::{ContactQuery, ContactRepository, StoredContact};

/// Request to create a new contact
#[derive(Debug)]
pub struct CreateContactInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub tags: Vec<String>,
    pub status: Option<ContactStatus>,
    pub company_id: Option<String>,
}

/// Request to update an existing contact
#[derive(Debug, Default)]
pub struct UpdateContactInput {
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

/// The Contact Service - your entry point for all contact operations
pub struct ContactService {
    repo: ContactRepository,
}

impl ContactService {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            repo: ContactRepository::new(db),
        }
    }

    /// Create a new contact
    ///
    /// This method:
    /// 1. Validates all input using domain validation
    /// 2. Checks email uniqueness (business rule requiring DB)
    /// 3. Creates the contact using ContactBuilder
    /// 4. Persists via repository
    pub async fn create(&self, input: CreateContactInput) -> AppResult<StoredContact> {
        // Step 1: Check email uniqueness BEFORE building
        // This is a business rule that requires database access
        if let Some(_existing) = self.repo.find_by_email(&input.email).await? {
            return Err(AppError::Conflict(format!(
                "A contact with email '{}' already exists",
                input.email
            )));
        }

        // Step 2: Build the contact using domain layer
        // This validates all fields and enforces business rules
        let mut builder = ContactBuilder::new()
            .first_name(&input.first_name)
            .last_name(&input.last_name)
            .email(&input.email);

        if let Some(ref phone) = input.phone {
            builder = builder.phone(phone);
        }

        if let Some(ref linkedin) = input.linkedin_url {
            builder = builder.linkedin_url(linkedin);
        }

        builder = builder.tags(input.tags);

        if let Some(status) = input.status {
            builder = builder.status(status);
        }

        if let Some(ref company_id) = input.company_id {
            builder = builder.company_id(company_id);
        }

        // Build validates everything
        let contact = builder.build()?;

        // Step 3: Persist
        let stored = self.repo.create_with_id(&contact).await?;

        Ok(stored)
    }

    /// Get a contact by ID
    pub async fn get(&self, id: &str) -> AppResult<StoredContact> {
        self.repo
            .find_by_id_with_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Contact '{}' not found", id)))
    }

    /// List contacts with optional filters
    pub async fn list(&self, query: ContactQuery) -> AppResult<Vec<StoredContact>> {
        // Get contacts from repository
        let contacts = self.repo.find_all(query).await?;

        // For now, we don't have IDs in find_all result
        // This is a simplification - in production you'd want IDs
        Ok(contacts
            .into_iter()
            .map(|c| StoredContact {
                id: String::new(), // Repository layer should include this
                contact: c,
            })
            .collect())
    }

    /// Update an existing contact
    ///
    /// This method:
    /// 1. Loads the existing contact
    /// 2. Validates each updated field
    /// 3. Checks email uniqueness if email changed
    /// 4. Applies updates using domain rules
    /// 5. Persists changes
    pub async fn update(&self, id: &str, input: UpdateContactInput) -> AppResult<StoredContact> {
        // Step 1: Load existing
        let stored = self
            .repo
            .find_by_id_with_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Contact '{}' not found", id)))?;

        let mut contact = stored.contact;

        // Step 2: Check email uniqueness if changing
        if let Some(ref new_email) = input.email {
            let normalized = new_email.trim().to_lowercase();
            if normalized != contact.email {
                if self.repo.email_exists_for_other(&normalized, id).await? {
                    return Err(AppError::Conflict(format!(
                        "A contact with email '{}' already exists",
                        normalized
                    )));
                }
                // Validate and update email
                crate::domain::validate_email(&normalized)?;
                contact.email = normalized;
            }
        }

        // Step 3: Apply other updates with validation
        if let Some(ref first_name) = input.first_name {
            crate::domain::validate_name(first_name, "first_name")?;
            contact.first_name = first_name.trim().to_string();
        }

        if let Some(ref last_name) = input.last_name {
            crate::domain::validate_name(last_name, "last_name")?;
            contact.last_name = last_name.trim().to_string();
        }

        if let Some(ref phone) = input.phone {
            crate::domain::validate_phone(Some(phone))?;
            contact.phone = if phone.is_empty() {
                None
            } else {
                Some(phone.clone())
            };
        }

        if let Some(ref linkedin) = input.linkedin_url {
            crate::domain::validate_linkedin_url(Some(linkedin))?;
            contact.linkedin_url = if linkedin.is_empty() {
                None
            } else {
                Some(linkedin.clone())
            };
        }

        if let Some(ref tags) = input.tags {
            contact.tags = crate::domain::validate_tags(tags)?;
        }

        if let Some(new_status) = input.status {
            // Use domain method which enforces transition rules
            contact.transition_status(new_status)?;
        }

        if let Some(score) = input.engagement_score {
            contact.update_engagement(score)?;
        }

        if let Some(ref company_id) = input.company_id {
            contact.company_id = if company_id.is_empty() {
                None
            } else {
                Some(company_id.clone())
            };
        }

        // Update timestamp
        contact.updated_at = chrono::Utc::now();

        // Step 4: Persist
        let updated = self.repo.update(id, &contact).await?;

        Ok(StoredContact {
            id: id.to_string(),
            contact: updated,
        })
    }

    /// Delete a contact
    pub async fn delete(&self, id: &str) -> AppResult<bool> {
        // Check exists first
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Contact '{}' not found", id)))?;

        self.repo.delete(id).await
    }

    /// Find a contact by email
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<Contact>> {
        self.repo.find_by_email(email).await
    }
}

#[cfg(test)]
mod tests {
    // Service tests would typically use a mock repository
    // For now, integration tests cover service behavior
}
