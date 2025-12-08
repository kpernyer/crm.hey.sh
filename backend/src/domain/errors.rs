//! Domain Errors - Business rule violations
//!
//! These errors represent BUSINESS problems, not technical ones.
//! "Invalid email format" is a domain error.
//! "Database connection failed" is NOT - that's infrastructure.

use std::fmt;

/// Errors that can occur when validating or processing domain objects
#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    /// A required field was empty or missing
    RequiredFieldMissing { field: String },

    /// A field value was invalid
    InvalidField { field: String, reason: String },

    /// A state transition was not allowed
    InvalidStateTransition { from: String, to: String, reason: String },

    /// A business rule was violated
    BusinessRuleViolation { rule: String, details: String },
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::RequiredFieldMissing { field } => {
                write!(f, "Required field '{}' is missing", field)
            }
            DomainError::InvalidField { field, reason } => {
                write!(f, "Invalid value for '{}': {}", field, reason)
            }
            DomainError::InvalidStateTransition { from, to, reason } => {
                write!(f, "Cannot transition from '{}' to '{}': {}", from, to, reason)
            }
            DomainError::BusinessRuleViolation { rule, details } => {
                write!(f, "Business rule '{}' violated: {}", rule, details)
            }
        }
    }
}

impl std::error::Error for DomainError {}

/// Result type for domain operations
pub type DomainResult<T> = Result<T, DomainError>;

// ============================================================================
// YOUR TURN: Understanding Exercise
// ============================================================================
//
// Q1: Why do we separate DomainError from database/HTTP errors?
//
// A1: _______________________________________________________________
//     _______________________________________________________________
//
// Q2: If a user tries to delete a contact who has active campaigns,
//     which error variant would you use? Why?
//
// A2: _______________________________________________________________
//     _______________________________________________________________
//
// Q3: Add a new error variant for "DuplicateEmail" below:
//
// YOUR CODE HERE:
//
