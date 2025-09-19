//! User domain models and validation logic

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

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
#[derive(Serialize, ToSchema, Debug, Clone, sqlx::FromRow)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// User's full name
    pub name: String,
    /// User's age in years
    pub age: i32,
    /// When the user was created
    pub created_at: DateTime<Utc>,
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

/// Pagination parameters for user queries
#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct PaginationParams {
    /// Pagination token from previous page (opaque cursor)
    pub next_token: Option<String>,
    /// Number of records to return (default: 200, max: 200)
    pub limit: Option<i32>,
}

/// Paginated response for users
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct PaginatedUsersResponse {
    /// List of users for this page
    pub users: Vec<User>,
    /// Token for the next page (opaque cursor)
    pub next_token: Option<String>,
    /// Whether there are more users available
    pub has_more: bool,
    /// Total number of users returned in this page
    pub count: usize,
}

/// Domain errors for user operations
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    /// Validation errors occurred during user data processing
    #[error("Validation failed")]
    ValidationError(Vec<ValidationError>),
    /// User was not found in the database
    #[error("User not found")]
    NotFound,
    /// Database operation failed
    #[error("Database error: {0}")]
    DatabaseError(String),
    /// Invalid pagination token
    #[error("Invalid pagination token")]
    InvalidToken,
}

