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
    #[must_use] pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a strict validation context
    #[must_use] pub fn strict() -> Self {
        Self {
            strict: true,
            ..Default::default()
        }
    }
    
    /// Adds metadata to the context
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Helper function to create a validation error
pub fn validation_error(message: impl Into<String>, field: Option<impl Into<String>>) -> ValidationError {
    ValidationError {
        message: message.into(),
        field: field.map(std::convert::Into::into),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_context_new() {
        let context = ValidationContext::new();
        assert!(!context.strict);
        assert!(context.metadata.is_empty());
    }

    #[test]
    fn test_validation_context_strict() {
        let context = ValidationContext::strict();
        assert!(context.strict);
        assert!(context.metadata.is_empty());
    }

    #[test]
    fn test_validation_context_with_metadata() {
        let context = ValidationContext::new()
            .with_metadata("key1", "value1")
            .with_metadata("key2", "value2");
        
        assert_eq!(context.metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.metadata.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_validation_error_creation() {
        let error = validation_error("Test message", Some("test_field"));
        assert_eq!(error.message, "Test message");
        assert_eq!(error.field, Some("test_field".to_string()));
    }

    #[test]
    fn test_field_error_creation() {
        let error = field_error("username", "Username is required");
        assert_eq!(error.message, "Username is required");
        assert_eq!(error.field, Some("username".to_string()));
    }

    #[test]
    fn test_general_error_creation() {
        let error = general_error("General validation failed");
        assert_eq!(error.message, "General validation failed");
        assert!(error.field.is_none());
    }
}