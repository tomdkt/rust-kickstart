//! # Rust Kickstart API
//!
//! A minimal REST API built with Axum, `PostgreSQL`, and comprehensive validation.
//! Provides user management endpoints with `OpenAPI` documentation.

#![allow(clippy::needless_for_each)]

use axum::{
    Json, Router,
    response::Html,
    routing::{get, post},
};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;

// Module declarations
pub mod user;
pub mod bank;
pub mod example;

// Re-export commonly used types
pub use user::{User, CreateUser, UpdateUser, UserService};
pub use bank::{BankService, BankError};



#[derive(OpenApi)]
#[openapi(
    paths(
        user::create_user_handler,
        user::get_all_users_handler,
        user::get_user_by_id_handler,
        user::update_user_handler,
        user::delete_user_handler
    ),
    components(schemas(
        user::CreateUser,
        user::UpdateUser,
        user::User,
        user::domain::ApiResponse,
        user::domain::ValidationError,
        user::domain::ValidationErrorResponse
    )),
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
    // Create services
    let user_service = UserService::new(pool.clone());
    let _bank_service = BankService::new(user_service.clone()); // Available for future use

    Router::new()
        .route("/", get(root_handler))
        .route("/users", post(user::create_user_handler).get(user::get_all_users_handler))
        .route(
            "/users/{id}",
            get(user::get_user_by_id_handler)
                .put(user::update_user_handler)
                .delete(user::delete_user_handler),
        )
        .route("/api-docs/openapi.json", get(serve_openapi))
        .route("/swagger-ui", get(serve_swagger_ui))
        .layer(TraceLayer::new_for_http())
        .with_state(user_service)
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
