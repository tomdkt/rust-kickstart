//! Integration tests for the User module HTTP endpoints
//! 
//! These tests verify the complete HTTP API functionality for user management,
//! testing the full stack from HTTP requests to database operations.

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::TestContext;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

#[tokio::test]
async fn test_get_all_users_empty() {
    // Arrange
    let ctx = TestContext::new().await;
    let request = Request::builder()
        .uri("/users")
        .body(Body::empty())
        .unwrap();

    // Act
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let paginated_response: Value = serde_json::from_slice(&body).unwrap();

    // Assert
    assert_eq!(status, StatusCode::OK, "Should return OK status");
    assert_eq!(paginated_response["users"].as_array().unwrap().len(), 0, "Users list should be empty initially");
    assert_eq!(paginated_response["count"], 0, "Count should be 0");
    assert_eq!(paginated_response["has_more"], false, "Should not have more pages");
    assert!(paginated_response["next_token"].is_null(), "Next token should be null");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_create_user() {
    // Arrange
    let ctx = TestContext::new().await;
    let new_user = json!({
        "name": "John Doe",
        "age": 30
    });
    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&new_user).unwrap()))
        .unwrap();

    // Act
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created_user: Value = serde_json::from_slice(&body).unwrap();

    // Assert
    assert_eq!(status, StatusCode::OK, "Should return OK status");
    assert_eq!(created_user["name"], "John Doe", "User name should match input");
    assert_eq!(created_user["age"], 30, "User age should match input");
    assert!(created_user["id"].is_number(), "User ID should be a number");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_user_not_found() {
    // Arrange
    let ctx = TestContext::new().await;
    let nonexistent_user_id = 999;
    let request = Request::builder()
        .uri(format!("/users/{nonexistent_user_id}"))
        .body(Body::empty())
        .unwrap();

    // Act
    let response = ctx.app.clone().oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND, "Should return NOT_FOUND for nonexistent user");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_full_crud_workflow() {
    // Arrange
    let ctx = TestContext::new().await;
    let original_user = json!({
        "name": "Alice Smith",
        "age": 28
    });
    let updated_user_data = json!({
        "name": "Alice Johnson",
        "age": 29
    });

    // Act & Assert - Create user
    let create_request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&original_user).unwrap()))
        .unwrap();

    let create_response = ctx.app.clone().oneshot(create_request).await.unwrap();
    let create_status = create_response.status();
    let create_body = create_response.into_body().collect().await.unwrap().to_bytes();
    let created_user: Value = serde_json::from_slice(&create_body).unwrap();
    let user_id = created_user["id"].as_i64().unwrap();

    assert_eq!(create_status, StatusCode::OK, "User creation should succeed");
    assert_eq!(created_user["name"], "Alice Smith", "Created user name should match");
    assert_eq!(created_user["age"], 28, "Created user age should match");

    // Act & Assert - Get user by ID
    let get_request = Request::builder()
        .uri(format!("/users/{user_id}"))
        .body(Body::empty())
        .unwrap();

    let get_response = ctx.app.clone().oneshot(get_request).await.unwrap();
    let get_status = get_response.status();
    let get_body = get_response.into_body().collect().await.unwrap().to_bytes();
    let retrieved_user: Value = serde_json::from_slice(&get_body).unwrap();

    assert_eq!(get_status, StatusCode::OK, "User retrieval should succeed");
    assert_eq!(retrieved_user["name"], "Alice Smith", "Retrieved user name should match");
    assert_eq!(retrieved_user["age"], 28, "Retrieved user age should match");

    // Act & Assert - Update user
    let update_request = Request::builder()
        .method("PUT")
        .uri(format!("/users/{user_id}"))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&updated_user_data).unwrap()))
        .unwrap();

    let update_response = ctx.app.clone().oneshot(update_request).await.unwrap();
    let update_status = update_response.status();
    let update_body = update_response.into_body().collect().await.unwrap().to_bytes();
    let updated_user: Value = serde_json::from_slice(&update_body).unwrap();

    assert_eq!(update_status, StatusCode::OK, "User update should succeed");
    assert_eq!(updated_user["name"], "Alice Johnson", "Updated user name should match");
    assert_eq!(updated_user["age"], 29, "Updated user age should match");

    // Act & Assert - Delete user
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(format!("/users/{user_id}"))
        .body(Body::empty())
        .unwrap();

    let delete_response = ctx.app.clone().oneshot(delete_request).await.unwrap();
    let delete_status = delete_response.status();
    assert_eq!(delete_status, StatusCode::OK, "User deletion should succeed");

    // Act & Assert - Verify user is deleted
    let verify_request = Request::builder()
        .uri(format!("/users/{user_id}"))
        .body(Body::empty())
        .unwrap();

    let verify_response = ctx.app.clone().oneshot(verify_request).await.unwrap();
    let verify_status = verify_response.status();
    assert_eq!(verify_status, StatusCode::NOT_FOUND, "Deleted user should not be found");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_pagination_basic() {
    // Arrange
    let ctx = TestContext::new().await;

    // Test basic pagination without tokens first
    let request = Request::builder()
        .uri("/users")
        .body(Body::empty())
        .unwrap();

    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let paginated_response: Value = serde_json::from_slice(&body).unwrap();
    
    // Should have paginated structure
    assert!(paginated_response.get("users").is_some());
    assert!(paginated_response.get("count").is_some());
    assert!(paginated_response.get("has_more").is_some());
    assert!(paginated_response.get("next_token").is_some());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_pagination_with_limit() {
    // Arrange
    let ctx = TestContext::new().await;

    // Test pagination with limit parameter
    let request = Request::builder()
        .uri("/users?limit=100")
        .body(Body::empty())
        .unwrap();

    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_invalid_pagination_token() {
    // Arrange
    let ctx = TestContext::new().await;

    // Test with invalid token
    let request = Request::builder()
        .uri("/users?next_token=invalid_token")
        .body(Body::empty())
        .unwrap();

    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}