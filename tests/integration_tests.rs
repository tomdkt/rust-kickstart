mod common;

use common::TestContext;
use serde_json::{json, Value};
use axum::http::{Request, StatusCode};
use axum::body::Body;
use tower::ServiceExt;
use http_body_util::BodyExt;
#[tokio::test]
async fn test_get_all_users_empty() {
    let ctx = TestContext::new().await;

    let request = Request::builder()
        .uri("/users")
        .body(Body::empty())
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let users: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert!(users.is_empty(), "Users list should be empty initially");
    
    // Explicit cleanup
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_create_user() {
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
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created_user: Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(created_user["name"], "John Doe");
    assert_eq!(created_user["age"], 30);
    assert!(created_user["id"].is_number());
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_user_not_found() {
    let ctx = TestContext::new().await;

    let request = Request::builder()
        .uri("/users/999")
        .body(Body::empty())
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_full_crud_workflow() {
    let ctx = TestContext::new().await;

    // 1. Create user
    let new_user = json!({
        "name": "Alice Smith",
        "age": 28
    });

    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&new_user).unwrap()))
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created_user: Value = serde_json::from_slice(&body).unwrap();
    let user_id = created_user["id"].as_i64().unwrap();

    // 2. Get user by ID
    let request = Request::builder()
        .uri(&format!("/users/{}", user_id))
        .body(Body::empty())
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let user: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(user["name"], "Alice Smith");
    assert_eq!(user["age"], 28);

    // 3. Update user
    let update_data = json!({
        "name": "Alice Johnson",
        "age": 29
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/users/{}", user_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&update_data).unwrap()))
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let updated_user: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(updated_user["name"], "Alice Johnson");
    assert_eq!(updated_user["age"], 29);

    // 4. Delete user
    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/users/{}", user_id))
        .body(Body::empty())
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Verify user is deleted
    let request = Request::builder()
        .uri(&format!("/users/{}", user_id))
        .body(Body::empty())
        .unwrap();
    
    let response = ctx.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    ctx.cleanup().await;
}