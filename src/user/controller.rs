//! User controller - HTTP handlers
//! 
//! This module is private to the user module and handles HTTP-specific concerns.
//! It should only be used internally by the user module's router setup.

use axum::{
    Json,
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{error, warn};

use super::domain::{User, CreateUser, UpdateUser, ValidationErrorResponse, ApiResponse, UserError, PaginationParams, PaginatedUsersResponse};


/// HTTP handler for creating a new user
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
#[tracing::instrument(skip(app_state, payload), fields(user_name = %payload.name, user_age = payload.age))]
pub async fn create_user_handler(
    State(app_state): State<crate::AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user_service = app_state.user_service;
    match user_service.create_user(payload).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(UserError::ValidationError(errors)) => {
            warn!(?errors, "Controller: Validation failed for create user");
            (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse { errors }),
            ).into_response()
        }
        Err(UserError::DatabaseError(msg)) => {
            error!(error = %msg, "Controller: Database error in create user");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(UserError::NotFound | UserError::InvalidToken) => {
            // These shouldn't happen in create, but handle them anyway
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// HTTP handler for retrieving users with optional pagination
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(
        ("next_token" = Option<String>, Query, description = "Pagination token from previous page (opaque cursor)"),
        ("limit" = Option<i32>, Query, description = "Number of records to return (default: 200, max: 200)")
    ),
    responses(
        (status = 200, description = "Paginated list of users", body = PaginatedUsersResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(skip(app_state), fields(next_token = params.next_token.as_deref(), limit = params.limit))]
pub async fn get_all_users_handler(
    State(app_state): State<crate::AppState>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let user_service = app_state.user_service;
    match user_service.get_users_paginated(params).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(UserError::InvalidToken) => {
            warn!("Controller: Invalid pagination token provided");
            StatusCode::BAD_REQUEST.into_response()
        }
        Err(UserError::DatabaseError(msg)) => {
            error!(error = %msg, "Controller: Database error in get users");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(_) => {
            // Other errors shouldn't happen in get_users, but handle them anyway
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}



/// HTTP handler for retrieving a specific user by ID
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
#[tracing::instrument(skip(app_state), fields(user_id = id))]
pub async fn get_user_by_id_handler(
    State(app_state): State<crate::AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let user_service = app_state.user_service;
    match user_service.get_user_by_id(id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(UserError::NotFound) => {
            warn!(user_id = id, "Controller: User not found");
            StatusCode::NOT_FOUND.into_response()
        }
        Err(UserError::DatabaseError(msg)) => {
            error!(error = %msg, user_id = id, "Controller: Database error in get user by id");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(_) => {
            // Other errors shouldn't happen here, but handle them anyway
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// HTTP handler for updating an existing user
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
#[tracing::instrument(skip(app_state, payload), fields(user_id = id, update_name = payload.name.as_deref(), update_age = payload.age))]
pub async fn update_user_handler(
    State(app_state): State<crate::AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let user_service = app_state.user_service;
    match user_service.update_user(id, payload).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(UserError::ValidationError(errors)) => {
            warn!(?errors, user_id = id, "Controller: Validation failed for update user");
            (
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse { errors }),
            ).into_response()
        }
        Err(UserError::NotFound) => {
            warn!(user_id = id, "Controller: User not found for update");
            StatusCode::NOT_FOUND.into_response()
        }
        Err(UserError::DatabaseError(msg)) => {
            error!(error = %msg, user_id = id, "Controller: Database error in update user");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(UserError::InvalidToken) => {
            // This shouldn't happen in update, but handle it anyway
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// HTTP handler for deleting a user
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
#[tracing::instrument(skip(app_state), fields(user_id = id))]
pub async fn delete_user_handler(
    State(app_state): State<crate::AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let user_service = app_state.user_service;
    match user_service.delete_user(id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(UserError::NotFound) => {
            warn!(user_id = id, "Controller: User not found for deletion");
            StatusCode::NOT_FOUND.into_response()
        }
        Err(UserError::DatabaseError(msg)) => {
            error!(error = %msg, user_id = id, "Controller: Database error in delete user");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(_) => {
            // Other errors shouldn't happen here, but handle them anyway
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}