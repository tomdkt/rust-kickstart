//! # Rust Kickstart API
//!
//! A minimal REST API built with Axum, `PostgreSQL`, and comprehensive validation.
//! Provides user management endpoints with `OpenAPI` documentation.

#![allow(clippy::needless_for_each)]
#[allow(unused_imports)]
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};
use utoipa::{OpenApi, ToSchema};

/// Request payload for creating a new user
#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateUser {
    /// User's full name
    pub name: String,
    /// User's age in years
    pub age: i32,
}

/// Request payload for updating an existing user
#[derive(Deserialize, ToSchema, Debug)]
pub struct UpdateUser {
    /// Updated user name (optional)
    pub name: Option<String>,
    /// Updated user age (optional)
    pub age: Option<i32>,
}

/// User entity returned by the API
#[derive(Serialize, ToSchema, Debug)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    /// User's full name
    pub name: String,
    /// User's age in years
    pub age: i32,
}

/// Generic API response with a message
#[derive(Serialize, ToSchema, Debug)]
pub struct ApiResponse {
    /// Response message
    pub message: String,
}

/// Individual validation error
#[derive(Serialize, ToSchema, Debug)]
pub struct ValidationError {
    /// Error message describing the validation failure
    pub message: String,
    /// Field name that caused the validation error (if applicable)
    pub field: Option<String>,
}

/// Response containing validation errors
#[derive(Serialize, ToSchema, Debug)]
pub struct ValidationErrorResponse {
    /// List of validation errors
    pub errors: Vec<ValidationError>,
}

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_all_users, get_user_by_id, update_user, delete_user),
    components(schemas(CreateUser, UpdateUser, User, ApiResponse, ValidationError, ValidationErrorResponse)),
    tags(
        (name = "users", description = "User management operations")
    ),
    info(
        title = "Rust Kickstart API",
        version = "0.1.0",
        description = "API for user management"
    )
)]
/// `OpenAPI` documentation structure
struct ApiDoc;

/// Validates user creation data
///
/// # Errors
/// Returns validation errors if:
/// - Name is empty or exceeds 100 characters
/// - Age is negative or exceeds 150 years
fn validate_create_user(user: &CreateUser) -> Result<(), Vec<ValidationError>> {
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
///
/// # Errors
/// Returns validation errors if:
/// - Name is empty or exceeds 100 characters (when provided)
/// - Age is negative or exceeds 150 years (when provided)
/// - No fields are provided for update
fn validate_update_user(user: &UpdateUser) -> Result<(), Vec<ValidationError>> {
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

/// Creates the main application router with database connection
///
/// # Panics
/// Panics if `DATABASE_URL` environment variable is not set or database connection fails
pub async fn create_app() -> Router {
    dotenvy::dotenv().ok(); // Don't panic in tests if .env is missing
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    info!("Database connection established");
    create_app_with_pool(pool)
}

/// Creates the application router with a provided database pool
pub fn create_app_with_pool(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user).get(get_all_users))
        .route(
            "/users/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
        .route("/api-docs/openapi.json", get(serve_openapi))
        .route("/swagger-ui", get(serve_swagger_ui))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUser,
    responses(
        (status = 200, description = "User created successfully", body = User),
        (status = 400, description = "Validation errors", body = ValidationErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
/// Creates a new user in the database
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, Json<ValidationErrorResponse>)> {
    info!(?payload, "Creating new user");

    // Validate input
    if let Err(validation_errors) = validate_create_user(&payload) {
        warn!(?validation_errors, "Validation failed for create user");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationErrorResponse {
                errors: validation_errors,
            }),
        ));
    }

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id, name, age",
        payload.name.trim(),
        payload.age
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to create user");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ValidationErrorResponse {
                errors: vec![ValidationError {
                    message: "Failed to create user".to_owned(),
                    field: None,
                }],
            }),
        )
    })?;

    info!(user_id = user.id, "User created successfully");
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    responses(
        (status = 200, description = "List of all users", body = Vec<User>),
        (status = 500, description = "Internal server error")
    )
)]
/// Retrieves all users from the database
async fn get_all_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    info!("Fetching all users");

    let users = sqlx::query_as!(User, "SELECT id, name, age FROM users ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to fetch users");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(count = users.len(), "Users fetched successfully");
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
/// Retrieves a specific user by ID
async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    info!(user_id = id, "Fetching user by ID");

    let user = sqlx::query_as!(User, "SELECT id, name, age FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!(error = %e, user_id = id, "Failed to fetch user");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(user) = user {
        info!(user_id = id, "User found");
        Ok(Json(user))
    } else {
        warn!(user_id = id, "User not found");
        Err(StatusCode::NOT_FOUND)
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 400, description = "Validation errors", body = ValidationErrorResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
/// Updates an existing user's information
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, Json<ValidationErrorResponse>)> {
    info!(user_id = id, ?payload, "Updating user");

    // Validate input
    if let Err(validation_errors) = validate_update_user(&payload) {
        warn!(?validation_errors, "Validation failed for update user");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationErrorResponse {
                errors: validation_errors,
            }),
        ));
    }

    // First check if user exists
    let existing_user = sqlx::query_as!(User, "SELECT id, name, age FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!(error = %e, user_id = id, "Failed to fetch user for update");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ValidationErrorResponse {
                    errors: vec![ValidationError {
                        message: "Failed to fetch user".to_owned(),
                        field: None,
                    }],
                }),
            )
        })?;

    let existing_user = existing_user.ok_or_else(|| {
        warn!(user_id = id, "User not found for update");
        (
            StatusCode::NOT_FOUND,
            Json(ValidationErrorResponse {
                errors: vec![ValidationError {
                    message: "User not found".to_owned(),
                    field: None,
                }],
            }),
        )
    })?;

    // Use existing values if not provided in update, trim name if provided
    let name = payload
        .name
        .as_ref()
        .map(|n| n.trim().to_owned())
        .unwrap_or(existing_user.name);
    let age = payload.age.unwrap_or(existing_user.age);

    let updated_user = sqlx::query_as!(
        User,
        "UPDATE users SET name = $1, age = $2 WHERE id = $3 RETURNING id, name, age",
        name,
        age,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!(error = %e, user_id = id, "Failed to update user");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ValidationErrorResponse {
                errors: vec![ValidationError {
                    message: "Failed to update user".to_owned(),
                    field: None,
                }],
            }),
        )
    })?;

    info!(user_id = id, "User updated successfully");
    Ok(Json(updated_user))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = ApiResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
/// Deletes a user from the database
async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse>, StatusCode> {
    info!(user_id = id, "Deleting user");

    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|e| {
            error!(error = %e, user_id = id, "Failed to delete user");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        warn!(user_id = id, "User not found for deletion");
        return Err(StatusCode::NOT_FOUND);
    }

    info!(user_id = id, "User deleted successfully");
    Ok(Json(ApiResponse {
        message: format!("User with id {id} deleted successfully"),
    }))
}

/// Serves the `OpenAPI` specification as JSON
async fn serve_openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// Root endpoint providing API information
async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Rust Kickstart API",
        "version": "0.1.0",
        "status": "running",
        "endpoints": {
            "users": "/users",
            "docs": "/swagger-ui",
            "openapi": "/api-docs/openapi.json"
        }
    }))
}

/// Serves the Swagger UI for API documentation
async fn serve_swagger_ui() -> Html<String> {
    Html(r#"<!DOCTYPE html>
<html>
<head>
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        SwaggerUIBundle({
            url: '/api-docs/openapi.json',
            dom_id: '#swagger-ui',
            presets: [
                SwaggerUIBundle.presets.apis,
                SwaggerUIBundle.presets.standalone
            ]
        });
    </script>
</body>
</html>"#.to_owned())
}
