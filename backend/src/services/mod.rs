//! Services module - business logic layer
//!
//! Services orchestrate:
//! - Domain validation (pure business rules)
//! - Repository operations (database I/O)
//! - Cross-cutting concerns (logging, metrics)
//!
//! Handlers call services. Services call domain + repository.

pub mod campaign_executor;
pub mod contact_service;
pub mod segment_builder;

pub use contact_service::*;
