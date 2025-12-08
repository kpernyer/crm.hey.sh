//! Domain Layer - Pure business logic, no I/O
//!
//! This is the CORE of your application. Everything here is:
//! - Pure functions (same input → same output)
//! - No database calls
//! - No HTTP
//! - No side effects
//! - Easily testable
//!
//! The domain layer defines WHAT the business rules are.
//! Other layers (handlers, repositories) define HOW to execute them.

pub mod contact;
pub mod validation;
pub mod engagement;
pub mod errors;

pub use contact::*;
pub use validation::*;
pub use engagement::*;
pub use errors::*;
