#[allow(unused_imports)]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::{info, warn, error};
use utoipa::{OpenApi, ToSchema};

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateUser {
    pub name: String,
    pub age: i32,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub age: Option<i32>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(create_user, get_all_users, get_user_by_id, update_user, delete_user),
    components(schemas(CreateUser, UpdateUser, User, ApiResponse)),
    tags(
        (name = "users", description = "User management operations")
    ),
    info(
        title = "Rust Kickstart API",
        version = "0.1.0",
        description = "API for user management"
    )
)]
struct ApiDoc;

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
    create_app_with_pool(pool).await
}

pub async fn create_app_with_pool(pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_all_users))
        .route("/users/{id}", get(get_user_by_id).put(update_user).delete(delete_user))
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
        (status = 400, description = "Invalid data")
    )
)]
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    info!(?payload, "Creating new user");
    
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id, name, age",
        payload.name,
        payload.age
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to create user");
        StatusCode::INTERNAL_SERVER_ERROR
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

    match user {
        Some(user) => {
            info!(user_id = id, "User found");
            Ok(Json(user))
        },
        None => {
            warn!(user_id = id, "User not found");
            Err(StatusCode::NOT_FOUND)
        },
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
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    info!(user_id = id, ?payload, "Updating user");
    
    // First check if user exists
    let existing_user = sqlx::query_as!(User, "SELECT id, name, age FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!(error = %e, user_id = id, "Failed to fetch user for update");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let existing_user = existing_user.ok_or_else(|| {
        warn!(user_id = id, "User not found for update");
        StatusCode::NOT_FOUND
    })?;

    // Use existing values if not provided in update
    let name = payload.name.unwrap_or(existing_user.name);
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
        StatusCode::INTERNAL_SERVER_ERROR
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
        message: format!("User with id {} deleted successfully", id),
    }))
}

async fn serve_openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

async fn serve_swagger_ui() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        SwaggerUIBundle({{
            url: '/api-docs/openapi.json',
            dom_id: '#swagger-ui',
            presets: [
                SwaggerUIBundle.presets.apis,
                SwaggerUIBundle.presets.standalone
            ]
        }});
    </script>
</body>
</html>"#
    ))
}