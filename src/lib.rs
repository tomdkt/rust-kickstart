//! # Rust Kickstart API
//!
//! A minimal REST API built with Axum, `PostgreSQL`, and comprehensive validation.
//! Provides user management endpoints with `OpenAPI` documentation.

#![allow(clippy::needless_for_each)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use axum::{
    response::Html, routing::{get, post},
    Json,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;
use utoipa::OpenApi;

// Module declarations
pub mod bank;
pub mod config;
pub mod health;
pub mod pagination;
pub mod user;

// Re-export commonly used types
pub use bank::{BankError, BankService};
pub use config::AppConfig;
pub use health::HealthService;
pub use user::{CreateUser, UpdateUser, User, UserService};

/// Application state containing all services
#[derive(Clone)]
pub struct AppState {
    /// User service for user management operations
    pub user_service: UserService,
    /// Health service for health check operations
    pub health_service: HealthService,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        user::create_user_handler,
        user::get_all_users_handler,
        user::get_user_by_id_handler,
        user::update_user_handler,
        user::delete_user_handler,
        health::health_check_handler,
        health::readiness_check_handler,
        health::liveness_check_handler
    ),
    components(schemas(
        user::CreateUser,
        user::UpdateUser,
        user::User,
        user::domain::ApiResponse,
        user::domain::ValidationError,
        user::domain::ValidationErrorResponse,
        user::domain::PaginationParams,
        user::domain::PaginatedUsersResponse,
        health::ComponentHealth,
        health::HealthCheckResponse
    )),
    tags(
        (name = "users", description = "User management operations"),
        (name = "health", description = "Health check and monitoring endpoints")
    ),
    info(
        title = "Rust Kickstart API",
        version = "0.1.0",
        description = "API for user management with comprehensive health monitoring"
    )
)]
/// `OpenAPI` documentation structure
struct ApiDoc;

/// Creates the main application router with database connection
///
/// # Panics
/// Panics if configuration cannot be loaded or database connection fails
pub async fn create_app() -> Router {
    let config = AppConfig::load();

    info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
        .expect("Failed to create pool");

    info!(
        "Database connection established with {} max connections",
        config.database.max_connections
    );
    create_app_with_pool(pool)
}

/// Creates the application router with a provided database pool
#[allow(clippy::needless_pass_by_value)]
pub fn create_app_with_pool(pool: PgPool) -> Router {
    // Create services
    let user_service = UserService::new(pool.clone());
    let health_service = HealthService::new(pool.clone());
    let _bank_service = BankService::new(user_service.clone()); // Available for future use

    let app_state = AppState {
        user_service,
        health_service,
    };

    Router::new()
        .route("/", get(root_handler))
        .route(
            "/users",
            post(user::create_user_handler).get(user::get_all_users_handler),
        )
        .route(
            "/users/{id}",
            get(user::get_user_by_id_handler)
                .put(user::update_user_handler)
                .delete(user::delete_user_handler),
        )
        .route("/health", get(health::health_check_handler))
        .route("/ready", get(health::readiness_check_handler))
        .route("/live", get(health::liveness_check_handler))
        .route("/api-docs/openapi.json", get(serve_openapi))
        .route("/swagger-ui", get(serve_swagger_ui))
        .layer(config::tracing::create_http_trace_layer())
        .with_state(app_state)
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
            "health": "/health",
            "readiness": "/ready",
            "liveness": "/live",
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
