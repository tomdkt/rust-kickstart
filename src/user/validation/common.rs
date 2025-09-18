//! Common validation utilities and types
//! 
//! Shared validation functionality used across different validation modules.

use crate::user::domain::ValidationError;

/// Type alias for validation results
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Validation context for passing additional information to validators
#[derive(Debug, Default)]
pub struct ValidationContext {
    /// Whether this is a strict validation (e.g., for API endpoints)
    pub strict: bool,
    /// Additional context data that validators might need
    pub metadata: std::collections::HashMap<String, String>,
}

impl ValidationContext {
    /// Creates a new validation context
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a strict validation context
    pub fn strict() -> Self {
        Self {
            strict: true,
            ..Default::default()
        }
    }
    
    /// Adds metadata to the context
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Helper function to create a validation error
pub fn validation_error(message: impl Into<String>, field: Option<impl Into<String>>) -> ValidationError {
    ValidationError {
        message: message.into(),
        field: field.map(|f| f.into()),
    }
}

/// Helper function to create a field-specific validation error
pub fn field_error(field: impl Into<String>, message: impl Into<String>) -> ValidationError {
    validation_error(message, Some(field))
}

/// Helper function to create a general validation error
pub fn general_error(message: impl Into<String>) -> ValidationError {
    validation_error(message, None::<String>)
}