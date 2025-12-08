//! Repository Layer - Database operations
//!
//! Repositories handle all database I/O. They:
//! - Convert between domain types and database types
//! - Execute queries
//! - Handle database errors
//!
//! Repositories know about SurrealDB. Domain layer does NOT.

pub mod contact_repository;

pub use contact_repository::*;
