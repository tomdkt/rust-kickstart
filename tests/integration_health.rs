//! Integration tests for health check endpoints
//!
//! Tests the health, readiness, and liveness endpoints to ensure they work correctly
//! and return appropriate status codes and response formats.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_health_check_endpoint() {
    let test_ctx = common::TestContext::new().await;
    let app = test_ctx.app.clone();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let health_response: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(health_response.get("status").is_some());
    assert!(health_response.get("version").is_some());
    assert!(health_response.get("timestamp").is_some());
    assert!(health_response.get("components").is_some());
    assert!(health_response.get("total_response_time_ms").is_some());

    // Verify components array
    let components = health_response["components"].as_array().unwrap();
    assert!(!components.is_empty());

    // Check for application and database components
    let component_names: Vec<&str> = components
        .iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();
    
    assert!(component_names.contains(&"application"));
    assert!(component_names.contains(&"database"));

    // Verify each component has required fields
    for component in components {
        assert!(component.get("name").is_some());
        assert!(component.get("status").is_some());
        assert!(component.get("response_time_ms").is_some());
    }
}

#[tokio::test]
async fn test_readiness_check_endpoint() {
    let test_ctx = common::TestContext::new().await;
    let app = test_ctx.app.clone();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let readiness_response: Value = serde_json::from_slice(&body).unwrap();

    // Should have same structure as health check
    assert!(readiness_response.get("status").is_some());
    assert!(readiness_response.get("version").is_some());
    assert!(readiness_response.get("timestamp").is_some());
    assert!(readiness_response.get("components").is_some());
}

#[tokio::test]
async fn test_liveness_check_endpoint() {
    let test_ctx = common::TestContext::new().await;
    let app = test_ctx.app.clone();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Liveness should always return 200 OK
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let liveness_response: Value = serde_json::from_slice(&body).unwrap();

    // Verify basic structure
    assert_eq!(liveness_response["status"], "healthy");
    assert!(liveness_response.get("version").is_some());
    assert!(liveness_response.get("timestamp").is_some());
    
    // Liveness should only check application component
    let components = liveness_response["components"].as_array().unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0]["name"], "application");
    assert_eq!(components[0]["status"], "healthy");
}

#[tokio::test]
async fn test_health_check_response_format() {
    let test_ctx = common::TestContext::new().await;
    let app = test_ctx.app.clone();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let health_response: Value = serde_json::from_slice(&body).unwrap();

    // Verify version matches Cargo.toml
    assert_eq!(health_response["version"], env!("CARGO_PKG_VERSION"));

    // Verify timestamp is in RFC3339 format
    let timestamp = health_response["timestamp"].as_str().unwrap();
    assert!(chrono::DateTime::parse_from_rfc3339(timestamp).is_ok());

    // Verify response time is a number
    assert!(health_response["total_response_time_ms"].is_number());
}

#[tokio::test]
async fn test_all_health_endpoints_in_root() {
    let test_ctx = common::TestContext::new().await;
    let app = test_ctx.app.clone();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let root_response: Value = serde_json::from_slice(&body).unwrap();

    let endpoints = &root_response["endpoints"];
    assert!(endpoints.get("health").is_some());
    assert!(endpoints.get("readiness").is_some());
    assert!(endpoints.get("liveness").is_some());
    
    assert_eq!(endpoints["health"], "/health");
    assert_eq!(endpoints["readiness"], "/ready");
    assert_eq!(endpoints["liveness"], "/live");
}