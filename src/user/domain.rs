//! User domain models and validation logic

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload for creating a new user
#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct CreateUser {
    /// User's full name
    pub name: String,
    /// User's age in years
    pub age: i32,
}

/// Request payload for updating an existing user
#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct UpdateUser {
    /// Updated user name (optional)
    pub name: Option<String>,
    /// Updated user age (optional)
    pub age: Option<i32>,
}

/// User entity returned by the API
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// User's full name
    pub name: String,
    /// User's age in years
    pub age: i32,
}

/// Individual validation error
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ValidationError {
    /// Error message describing the validation failure
    pub message: String,
    /// Field name that caused the validation error (if applicable)
    pub field: Option<String>,
}

/// Response containing validation errors
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ValidationErrorResponse {
    /// List of validation errors
    pub errors: Vec<ValidationError>,
}

/// Generic API response with a message
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ApiResponse {
    /// Response message
    pub message: String,
}

/// Domain errors for user operations
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Validation failed")]
    ValidationError(Vec<ValidationError>),
    #[error("User not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Validates user creation data
pub fn validate_create_user(user: &CreateUser) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate name
    if user.name.trim().is_empty() {
        errors.push(ValidationError {
            message: "Name cannot be empty".to_owned(),
            field: Some("name".to_owned()),
        });
    }

    if user.name.len() > 100 {
        errors.push(ValidationError {
            message: "Name cannot exceed 100 characters".to_owned(),
            field: Some("name".to_owned()),
        });
    }

    // Validate age
    if user.age < 0 {
        errors.push(ValidationError {
            message: "Age cannot be negative".to_owned(),
            field: Some("age".to_owned()),
        });
    }

    if user.age > 150 {
        errors.push(ValidationError {
            message: "Age cannot exceed 150 years".to_owned(),
            field: Some("age".to_owned()),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates user update data
pub fn validate_update_user(user: &UpdateUser) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate name if provided
    if let Some(ref name) = user.name {
        if name.trim().is_empty() {
            errors.push(ValidationError {
                message: "Name cannot be empty".to_owned(),
                field: Some("name".to_owned()),
            });
        }

        if name.len() > 100 {
            errors.push(ValidationError {
                message: "Name cannot exceed 100 characters".to_owned(),
                field: Some("name".to_owned()),
            });
        }
    }

    // Validate age if provided
    if let Some(age) = user.age {
        if age < 0 {
            errors.push(ValidationError {
                message: "Age cannot be negative".to_owned(),
                field: Some("age".to_owned()),
            });
        }

        if age > 150 {
            errors.push(ValidationError {
                message: "Age cannot exceed 150 years".to_owned(),
                field: Some("age".to_owned()),
            });
        }
    }

    // Check if at least one field is provided for update
    if user.name.is_none() && user.age.is_none() {
        errors.push(ValidationError {
            message: "At least one field (name or age) must be provided for update".to_owned(),
            field: None,
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}