//! Contact Domain - The core Contact entity and its business rules
//!
//! This module defines:
//! 1. The Contact entity (what data it holds)
//! 2. Contact status and transitions (state machine)
//! 3. Factory functions for creating valid contacts
//! 4. Business rules for contact operations
//!
//! IMPORTANT: No database code here. No IDs from storage.
//! This is the IDEAL contact as the business sees it.

use super::errors::{DomainError, DomainResult};
use super::validation::{
    validate_email, validate_linkedin_url, validate_name, validate_phone, validate_tags,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Contact Status - A State Machine
// ============================================================================

/// The lifecycle status of a contact
///
/// This is a STATE MACHINE. Not all transitions are valid:
///
/// ```text
///                    ┌──────────────┐
///                    │              │
///     ┌──────────────►    Lead      ◄────────────────┐
///     │              │              │                │
///     │              └──────┬───────┘                │
///     │                     │                        │
///     │                     ▼                        │
///     │              ┌──────────────┐                │
///     │              │              │                │
///     │              │   Customer   │                │
///     │              │              │                │
///     │              └──────┬───────┘                │
///     │                     │                        │
///     │         ┌───────────┼───────────┐            │
///     │         │           │           │            │
///     │         ▼           ▼           ▼            │
///     │    ┌─────────┐ ┌─────────┐ ┌─────────┐       │
///     │    │ Partner │ │Investor │ │  Other  │───────┘
///     │    └─────────┘ └─────────┘ └─────────┘
///     │         │           │
///     └─────────┴───────────┘
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactStatus {
    /// Initial state - someone we're trying to convert
    Lead,
    /// Converted - they're paying us or using our product
    Customer,
    /// Strategic relationship - mutual benefit
    Partner,
    /// Financial relationship - they've invested
    Investor,
    /// Catch-all for contacts that don't fit other categories
    Other,
}

impl ContactStatus {
    /// Check if a status transition is valid
    ///
    /// # Business Rules:
    /// - Lead can become anything
    /// - Customer can become Partner, Investor, or back to Lead (churned)
    /// - Partner can become Customer, Investor, or Lead
    /// - Investor can become Customer, Partner, or Lead
    /// - Other can become anything
    /// - Any status can become Other
    pub fn can_transition_to(&self, new_status: ContactStatus) -> bool {
        use ContactStatus::*;

        // Same status is always "valid" (no-op)
        if *self == new_status {
            return true;
        }

        // Any status can become Other, and Other can become anything
        if new_status == Other || *self == Other {
            return true;
        }

        match (self, new_status) {
            // Lead is the entry point - can go anywhere
            (Lead, _) => true,

            // Customer transitions
            (Customer, Lead) => true,     // Churned
            (Customer, Partner) => true,  // Became strategic partner
            (Customer, Investor) => true, // Invested in us

            // Partner transitions
            (Partner, Lead) => true,     // Relationship ended
            (Partner, Customer) => true, // Also became customer
            (Partner, Investor) => true, // Also invested

            // Investor transitions
            (Investor, Lead) => true,     // Relationship ended
            (Investor, Customer) => true, // Also became customer
            (Investor, Partner) => true,  // Also became partner

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Get a human-readable explanation for why a transition is/isn't allowed
    pub fn transition_explanation(&self, new_status: ContactStatus) -> &'static str {
        use ContactStatus::*;

        if self.can_transition_to(new_status) {
            return "Transition allowed";
        }

        match (self, new_status) {
            (Customer, Lead) => "Use 'churned' workflow instead of direct status change",
            (Partner, Lead) => "Use 'end partnership' workflow instead",
            (Investor, Lead) => "Use 'investor exit' workflow instead",
            _ => "This status transition is not allowed by business rules",
        }
    }
}

impl Default for ContactStatus {
    fn default() -> Self {
        ContactStatus::Lead
    }
}

impl std::fmt::Display for ContactStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactStatus::Lead => write!(f, "Lead"),
            ContactStatus::Customer => write!(f, "Customer"),
            ContactStatus::Partner => write!(f, "Partner"),
            ContactStatus::Investor => write!(f, "Investor"),
            ContactStatus::Other => write!(f, "Other"),
        }
    }
}

// ============================================================================
// Contact Entity
// ============================================================================

/// A Contact in the CRM system
///
/// This represents a VALID contact. You cannot create an invalid Contact
/// directly - you must use the builder or factory functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    // Required fields
    pub first_name: String,
    pub last_name: String,
    pub email: String,

    // Optional fields
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,

    // Classification
    pub tags: Vec<String>,
    pub status: ContactStatus,

    // Metrics
    pub engagement_score: f64,

    // Relationships (IDs, resolved by repository layer)
    pub company_id: Option<String>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    /// Get the full name of the contact
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Check if the contact has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        let normalized = tag.to_lowercase();
        self.tags.iter().any(|t| t == &normalized)
    }

    /// Check if the contact is considered "engaged"
    ///
    /// Business rule: engagement score >= 50 is considered engaged
    pub fn is_engaged(&self) -> bool {
        self.engagement_score >= 50.0
    }

    /// Check if the contact is considered "at risk"
    ///
    /// Business rule: Customer with engagement < 30 is at risk of churning
    pub fn is_at_risk(&self) -> bool {
        self.status == ContactStatus::Customer && self.engagement_score < 30.0
    }

    /// Attempt to transition to a new status
    pub fn transition_status(&mut self, new_status: ContactStatus) -> DomainResult<()> {
        if !self.status.can_transition_to(new_status) {
            return Err(DomainError::InvalidStateTransition {
                from: self.status.to_string(),
                to: new_status.to_string(),
                reason: self.status.transition_explanation(new_status).to_string(),
            });
        }

        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Add a tag to the contact
    pub fn add_tag(&mut self, tag: &str) -> DomainResult<()> {
        let validated = super::validation::validate_tag(tag)?;

        if !self.tags.contains(&validated) {
            self.tags.push(validated);
            self.updated_at = Utc::now();
        }

        Ok(())
    }

    /// Remove a tag from the contact
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let normalized = tag.to_lowercase();
        let initial_len = self.tags.len();
        self.tags.retain(|t| t != &normalized);

        if self.tags.len() != initial_len {
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Update engagement score
    pub fn update_engagement(&mut self, new_score: f64) -> DomainResult<()> {
        if new_score.is_nan() || new_score.is_infinite() {
            return Err(DomainError::InvalidField {
                field: "engagement_score".to_string(),
                reason: "Score must be a finite number".to_string(),
            });
        }

        // Clamp to valid range
        self.engagement_score = new_score.clamp(0.0, 100.0);
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ============================================================================
// Contact Builder - The safe way to create contacts
// ============================================================================

/// Builder for creating valid Contact instances
///
/// This ensures all validation rules are enforced before a Contact is created.
///
/// # Example
/// ```
/// use crm_backend::domain::contact::ContactBuilder;
///
/// let contact = ContactBuilder::new()
///     .first_name("John")
///     .last_name("Doe")
///     .email("john@example.com")
///     .phone("+1 555 123 4567")
///     .tag("vip")
///     .tag("early-adopter")
///     .build()
///     .expect("Valid contact");
/// ```
#[derive(Default)]
pub struct ContactBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    linkedin_url: Option<String>,
    tags: Vec<String>,
    status: ContactStatus,
    company_id: Option<String>,
}

impl ContactBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first_name(mut self, name: &str) -> Self {
        self.first_name = Some(name.trim().to_string());
        self
    }

    pub fn last_name(mut self, name: &str) -> Self {
        self.last_name = Some(name.trim().to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.trim().to_lowercase());
        self
    }

    pub fn phone(mut self, phone: &str) -> Self {
        let trimmed = phone.trim();
        if !trimmed.is_empty() {
            self.phone = Some(trimmed.to_string());
        }
        self
    }

    pub fn linkedin_url(mut self, url: &str) -> Self {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            self.linkedin_url = Some(trimmed.to_string());
        }
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    pub fn status(mut self, status: ContactStatus) -> Self {
        self.status = status;
        self
    }

    pub fn company_id(mut self, id: &str) -> Self {
        self.company_id = Some(id.to_string());
        self
    }

    /// Build the Contact, validating all fields
    pub fn build(self) -> DomainResult<Contact> {
        // Validate required fields
        let first_name = self.first_name.ok_or_else(|| DomainError::RequiredFieldMissing {
            field: "first_name".to_string(),
        })?;
        validate_name(&first_name, "first_name")?;

        let last_name = self.last_name.ok_or_else(|| DomainError::RequiredFieldMissing {
            field: "last_name".to_string(),
        })?;
        validate_name(&last_name, "last_name")?;

        let email = self.email.ok_or_else(|| DomainError::RequiredFieldMissing {
            field: "email".to_string(),
        })?;
        validate_email(&email)?;

        // Validate optional fields
        validate_phone(self.phone.as_deref())?;
        validate_linkedin_url(self.linkedin_url.as_deref())?;

        // Validate and normalize tags
        let tags = validate_tags(&self.tags)?;

        let now = Utc::now();

        Ok(Contact {
            first_name,
            last_name,
            email,
            phone: self.phone,
            linkedin_url: self.linkedin_url,
            tags,
            status: self.status,
            engagement_score: 0.0, // New contacts start at 0
            company_id: self.company_id,
            created_at: now,
            updated_at: now,
        })
    }
}

// ============================================================================
// YOUR TURN: Implement ContactUpdater
// ============================================================================

/// Updater for modifying existing contacts
///
/// Unlike ContactBuilder, this takes an existing contact and applies
/// partial updates to it.
///
/// # Example
/// ```
/// let updated = ContactUpdater::new(existing_contact)
///     .email("new@example.com")
///     .add_tag("priority")
///     .apply()?;
/// ```
pub struct ContactUpdater {
    contact: Contact,
    // Track what fields were modified
    modified_fields: Vec<String>,
}

impl ContactUpdater {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            modified_fields: Vec::new(),
        }
    }

    /// Update email address
    ///
    /// YOUR IMPLEMENTATION:
    pub fn email(mut self, email: &str) -> DomainResult<Self> {
        // TODO: Implement this
        //
        // Steps:
        // 1. Validate the new email
        // 2. Update self.contact.email
        // 3. Add "email" to modified_fields
        // 4. Update updated_at timestamp
        // 5. Return self for chaining

        todo!("Implement email update")
    }

    /// Update phone number
    ///
    /// YOUR IMPLEMENTATION:
    pub fn phone(mut self, phone: Option<&str>) -> DomainResult<Self> {
        // TODO: Implement this

        todo!("Implement phone update")
    }

    /// Add a tag
    ///
    /// YOUR IMPLEMENTATION:
    pub fn add_tag(mut self, tag: &str) -> DomainResult<Self> {
        // TODO: Implement this
        // Hint: Use self.contact.add_tag()

        todo!("Implement add_tag")
    }

    /// Change status
    ///
    /// YOUR IMPLEMENTATION:
    pub fn status(mut self, new_status: ContactStatus) -> DomainResult<Self> {
        // TODO: Implement this
        // Hint: Use self.contact.transition_status()

        todo!("Implement status change")
    }

    /// Apply all changes and return the updated contact
    pub fn apply(self) -> DomainResult<Contact> {
        // The contact was already updated in place by the builder methods
        Ok(self.contact)
    }

    /// Get list of modified fields (useful for audit logging)
    pub fn modified_fields(&self) -> &[String] {
        &self.modified_fields
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- ContactBuilder Tests ----

    #[test]
    fn test_build_valid_contact() {
        let contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .build()
            .unwrap();

        assert_eq!(contact.first_name, "John");
        assert_eq!(contact.last_name, "Doe");
        assert_eq!(contact.email, "john@example.com");
        assert_eq!(contact.status, ContactStatus::Lead);
        assert_eq!(contact.engagement_score, 0.0);
    }

    #[test]
    fn test_build_contact_with_all_fields() {
        let contact = ContactBuilder::new()
            .first_name("Jane")
            .last_name("Smith")
            .email("jane@example.com")
            .phone("+1 555 123 4567")
            .linkedin_url("https://linkedin.com/in/janesmith")
            .tag("VIP")
            .tag("Early-Adopter")
            .status(ContactStatus::Customer)
            .company_id("company-123")
            .build()
            .unwrap();

        assert_eq!(contact.phone, Some("+1 555 123 4567".to_string()));
        assert_eq!(
            contact.linkedin_url,
            Some("https://linkedin.com/in/janesmith".to_string())
        );
        assert_eq!(contact.tags, vec!["vip", "early-adopter"]); // Normalized
        assert_eq!(contact.status, ContactStatus::Customer);
        assert_eq!(contact.company_id, Some("company-123".to_string()));
    }

    #[test]
    fn test_build_missing_required_fields() {
        let result = ContactBuilder::new()
            .first_name("John")
            // Missing last_name and email
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_build_invalid_email() {
        let result = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("invalid-email")
            .build();

        assert!(result.is_err());
    }

    // ---- Status Transition Tests ----

    #[test]
    fn test_valid_status_transitions() {
        use ContactStatus::*;

        // Lead can go anywhere
        assert!(Lead.can_transition_to(Customer));
        assert!(Lead.can_transition_to(Partner));
        assert!(Lead.can_transition_to(Investor));
        assert!(Lead.can_transition_to(Other));

        // Customer can become Lead (churn)
        assert!(Customer.can_transition_to(Lead));
        assert!(Customer.can_transition_to(Partner));
        assert!(Customer.can_transition_to(Investor));

        // Any can become Other
        assert!(Customer.can_transition_to(Other));
        assert!(Partner.can_transition_to(Other));
        assert!(Investor.can_transition_to(Other));
    }

    #[test]
    fn test_same_status_transition() {
        use ContactStatus::*;

        // Same status is always valid (no-op)
        assert!(Lead.can_transition_to(Lead));
        assert!(Customer.can_transition_to(Customer));
    }

    #[test]
    fn test_contact_transition_status() {
        let mut contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .build()
            .unwrap();

        // Lead -> Customer: valid
        assert!(contact.transition_status(ContactStatus::Customer).is_ok());
        assert_eq!(contact.status, ContactStatus::Customer);

        // Customer -> Partner: valid
        assert!(contact.transition_status(ContactStatus::Partner).is_ok());
        assert_eq!(contact.status, ContactStatus::Partner);
    }

    // ---- Helper Method Tests ----

    #[test]
    fn test_full_name() {
        let contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .build()
            .unwrap();

        assert_eq!(contact.full_name(), "John Doe");
    }

    #[test]
    fn test_has_tag() {
        let contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .tag("vip")
            .build()
            .unwrap();

        assert!(contact.has_tag("vip"));
        assert!(contact.has_tag("VIP")); // Case insensitive
        assert!(!contact.has_tag("other"));
    }

    #[test]
    fn test_add_remove_tag() {
        let mut contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .build()
            .unwrap();

        // Add tag
        contact.add_tag("priority").unwrap();
        assert!(contact.has_tag("priority"));

        // Add duplicate (should not duplicate)
        contact.add_tag("PRIORITY").unwrap();
        assert_eq!(contact.tags.len(), 1);

        // Remove tag
        assert!(contact.remove_tag("priority"));
        assert!(!contact.has_tag("priority"));

        // Remove non-existent (should return false)
        assert!(!contact.remove_tag("nonexistent"));
    }

    #[test]
    fn test_engagement_levels() {
        let mut contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .status(ContactStatus::Customer)
            .build()
            .unwrap();

        // Low engagement - at risk
        contact.update_engagement(20.0).unwrap();
        assert!(!contact.is_engaged());
        assert!(contact.is_at_risk());

        // High engagement - not at risk
        contact.update_engagement(80.0).unwrap();
        assert!(contact.is_engaged());
        assert!(!contact.is_at_risk());
    }

    #[test]
    fn test_engagement_clamping() {
        let mut contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .build()
            .unwrap();

        // Should clamp to 100
        contact.update_engagement(150.0).unwrap();
        assert_eq!(contact.engagement_score, 100.0);

        // Should clamp to 0
        contact.update_engagement(-50.0).unwrap();
        assert_eq!(contact.engagement_score, 0.0);
    }

    // ---- YOUR TESTS: ContactUpdater ----

    #[test]
    #[ignore] // Remove this line after implementing ContactUpdater
    fn test_contact_updater_email() {
        let contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("old@example.com")
            .build()
            .unwrap();

        let updated = ContactUpdater::new(contact)
            .email("new@example.com")
            .unwrap()
            .apply()
            .unwrap();

        assert_eq!(updated.email, "new@example.com");
    }

    #[test]
    #[ignore] // Remove this line after implementing ContactUpdater
    fn test_contact_updater_status_transition() {
        let contact = ContactBuilder::new()
            .first_name("John")
            .last_name("Doe")
            .email("john@example.com")
            .build()
            .unwrap();

        let updated = ContactUpdater::new(contact)
            .status(ContactStatus::Customer)
            .unwrap()
            .apply()
            .unwrap();

        assert_eq!(updated.status, ContactStatus::Customer);
    }
}
