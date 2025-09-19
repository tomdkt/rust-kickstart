//! Health check module
//!
//! Provides comprehensive health checking functionality for the application,
//! including database connectivity and overall system status.
//! This module is completely independent and doesn't depend on other business modules.

use axum::{extract::State, http::StatusCode, response::Json};
use serde::Serialize;
use sqlx::PgPool;
use std::time::Instant;
use tracing::{error, info};
use utoipa::ToSchema;

/// Safely converts duration to milliseconds as u64, capping at `u64::MAX`
#[allow(clippy::cast_possible_truncation)]
fn duration_to_millis(duration: std::time::Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

/// Health check status for individual components
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ComponentHealth {
    /// Component name (e.g., "database", "application")
    pub name: String,
    /// Health status ("healthy", "unhealthy", "degraded")
    pub status: String,
    /// Optional error message if unhealthy
    pub message: Option<String>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

/// Overall application health check response
#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct HealthCheckResponse {
    /// Overall application status
    pub status: String,
    /// Application version
    pub version: String,
    /// Timestamp of the health check
    pub timestamp: String,
    /// Individual component health statuses
    pub components: Vec<ComponentHealth>,
    /// Total response time in milliseconds
    pub total_response_time_ms: u64,
}

/// Health check errors
#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    /// Database connection or query failed
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Health repository for performing health checks
#[derive(Clone)]
pub struct HealthRepository {
    pool: PgPool,
}

impl HealthRepository {
    /// Creates a new `HealthRepository` instance
    #[must_use] pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Performs a database health check
    pub async fn check_database(&self) -> Result<(), HealthError> {
        info!("Performing database health check");

        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "Database health check failed");
                HealthError::DatabaseError(e.to_string())
            })?;

        info!("Database health check passed");
        Ok(())
    }

    /// Performs a more comprehensive database health check
    pub async fn check_database_detailed(&self) -> Result<ComponentHealth, HealthError> {
        let start_time = Instant::now();
        
        match self.check_database().await {
            Ok(()) => Ok(ComponentHealth {
                name: "database".to_owned(),
                status: "healthy".to_owned(),
                message: None,
                response_time_ms: duration_to_millis(start_time.elapsed()),
            }),
            Err(e) => {
                error!(error = %e, "Database health check failed");
                Ok(ComponentHealth {
                    name: "database".to_owned(),
                    status: "unhealthy".to_owned(),
                    message: Some(e.to_string()),
                    response_time_ms: duration_to_millis(start_time.elapsed()),
                })
            }
        }
    }
}

/// Health service that coordinates health checks
#[derive(Clone)]
pub struct HealthService {
    repository: HealthRepository,
}

impl HealthService {
    /// Creates a new `HealthService` instance
    #[must_use] pub fn new(pool: PgPool) -> Self {
        Self {
            repository: HealthRepository::new(pool),
        }
    }

    /// Performs a complete health check of all components
    #[tracing::instrument(skip(self))]
    pub async fn check_health(&self) -> HealthCheckResponse {
        let start_time = Instant::now();
        info!("Starting comprehensive health check");

        let mut components = Vec::new();
        let mut overall_healthy = true;

        // Check application status (always healthy if we reach this point)
        let app_start = Instant::now();
        let app_health = ComponentHealth {
            name: "application".to_owned(),
            status: "healthy".to_owned(),
            message: None,
            response_time_ms: duration_to_millis(app_start.elapsed()),
        };
        components.push(app_health);

        // Check database status
        let db_health = self.repository.check_database_detailed().await
            .unwrap_or_else(|e| {
                error!(error = %e, "Failed to perform database health check");
                overall_healthy = false;
                ComponentHealth {
                    name: "database".to_owned(),
                    status: "unhealthy".to_owned(),
                    message: Some(format!("Health check failed: {e}")),
                    response_time_ms: 0,
                }
            });

        if db_health.status != "healthy" {
            overall_healthy = false;
        }
        components.push(db_health);

        let total_time = duration_to_millis(start_time.elapsed());
        let overall_status = if overall_healthy { "healthy" } else { "unhealthy" };

        let response = HealthCheckResponse {
            status: overall_status.to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            components,
            total_response_time_ms: total_time,
        };

        info!(
            status = overall_status,
            response_time_ms = total_time,
            "Health check completed"
        );

        response
    }

    /// Performs a lightweight liveness check (application only)
    #[must_use] pub fn check_liveness(&self) -> HealthCheckResponse {
        let start_time = Instant::now();

        let app_health = ComponentHealth {
            name: "application".to_owned(),
            status: "healthy".to_owned(),
            message: None,
            response_time_ms: duration_to_millis(start_time.elapsed()),
        };

        let total_time = duration_to_millis(start_time.elapsed());

        HealthCheckResponse {
            status: "healthy".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            components: vec![app_health],
            total_response_time_ms: total_time,
        }
    }
}

/// Health check handler that verifies application and database status
///
/// Returns HTTP 200 if all components are healthy, HTTP 503 if any component is unhealthy.
/// Follows industry standards for health check endpoints.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "All components are healthy", body = HealthCheckResponse),
        (status = 503, description = "One or more components are unhealthy", body = HealthCheckResponse)
    ),
    tag = "health"
)]
#[tracing::instrument(skip(app_state))]
pub async fn health_check_handler(
    State(app_state): State<crate::AppState>,
) -> Result<Json<HealthCheckResponse>, (StatusCode, Json<HealthCheckResponse>)> {
    let health_service = app_state.health_service;
    let response = health_service.check_health().await;

    if response.status == "healthy" {
        Ok(Json(response))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, Json(response)))
    }
}

/// Readiness check handler for Kubernetes-style probes
///
/// Similar to health check but focuses on whether the service is ready to accept traffic.
/// Returns HTTP 200 if ready, HTTP 503 if not ready.
#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Service is ready to accept traffic", body = HealthCheckResponse),
        (status = 503, description = "Service is not ready", body = HealthCheckResponse)
    ),
    tag = "health"
)]
pub async fn readiness_check_handler(
    State(app_state): State<crate::AppState>,
) -> Result<Json<HealthCheckResponse>, (StatusCode, Json<HealthCheckResponse>)> {
    let health_service = app_state.health_service;
    // For now, readiness is the same as health check
    // In more complex applications, this might check additional conditions
    // like cache warmup, external service dependencies, etc.
    let response = health_service.check_health().await;

    if response.status == "healthy" {
        Ok(Json(response))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, Json(response)))
    }
}

/// Liveness check handler for Kubernetes-style probes
///
/// Simple check to verify the application is running and responsive.
/// Should be lightweight and not depend on external services.
#[utoipa::path(
    get,
    path = "/live",
    responses(
        (status = 200, description = "Service is alive", body = HealthCheckResponse)
    ),
    tag = "health"
)]
pub async fn liveness_check_handler(State(app_state): State<crate::AppState>) -> Json<HealthCheckResponse> {
    let health_service = app_state.health_service;
    let response = health_service.check_liveness();
    Json(response)
}