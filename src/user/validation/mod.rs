//! User validation module
//! 
//! This module contains all validation logic for user operations.
//! It's organized by operation type and includes common validation utilities.

pub mod create;
pub mod update;
pub mod common;
pub mod rules;

// Re-export main validation functions for easy access
pub use create::validate_create_user;
pub use update::validate_update_user;
pub use common::{ValidationResult, ValidationContext};
pub use rules::*;