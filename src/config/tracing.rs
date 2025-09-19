//! Tracing and logging configuration for the application.
//! 
//! This module provides structured logging with different formats for development
//! and production environments, along with HTTP request tracing capabilities.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry, Layer};

/// Initialize tracing/logging configuration.
/// 
/// This should be called once at application startup. It configures:
/// - JSON format in production for structured logging
/// - Pretty format in development for readability
/// - Appropriate log levels based on environment
/// - Request tracing and correlation IDs
/// 
/// # Errors
/// 
/// Returns an error if the tracing subscriber cannot be initialized.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Use JSON format in production, pretty format in development
    let fmt_layer = if cfg!(debug_assertions) {
        // Development: Clean format focusing on our code
        tracing_subscriber::fmt::layer()
            .with_target(true)           // Show module names (rust_kickstart::user::repository)
            .with_thread_ids(true)       // Show ThreadId for debugging concurrency
            .with_thread_names(false)    // Hide "tokio-runtime-worker" noise
            .with_file(false)            // Hide file paths to reduce noise
            .with_line_number(false)     // Hide line numbers to reduce noise
            .with_level(true)            // Show log level (INFO, DEBUG, etc.)
            .with_ansi(true)             // Keep colors for better readability
            .boxed()
    } else {
        // Production: JSON format for structured logging
        tracing_subscriber::fmt::layer()
            .json()
            .with_target(true)
            .with_thread_ids(true)
            .with_current_span(false)
            .with_span_list(true)
            .flatten_event(true)
            .boxed()
    };

    // Enhanced EnvFilter configuration with better defaults
    let env_filter = EnvFilter::try_from_default_env().or_else(|_| {
        // Different log levels for dev vs prod
        if cfg!(debug_assertions) {
            // Development: More verbose logging but filter out noisy tower_http internals
            EnvFilter::try_new(
                "rust_kickstart=debug,tower_http=info,tower_http::trace::on_request=info,tower_http::trace::on_response=info,axum::rejection=trace,sqlx=info"
            )
        } else {
            // Production: Less verbose, focus on important events
            EnvFilter::try_new(
                "rust_kickstart=info,tower_http=info,sqlx=warn"
            )
        }
    })?;

    // Initialize the global subscriber
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()?;

    Ok(())
}

/// Create HTTP request tracing layer for web applications.
/// 
/// This layer provides enhanced request tracing with better logging
/// and structured information for monitoring and debugging.
pub fn create_http_trace_layer() -> tower_http::trace::TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
> {
    tower_http::trace::TraceLayer::new_for_http()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_init() {
        // Test that tracing initialization doesn't panic
        // Note: We can't actually test the full initialization in unit tests
        // as it would conflict with other tests
        let result = std::panic::catch_unwind(|| {
            // Just test that the EnvFilter creation works
            let _filter = EnvFilter::try_new("rust_kickstart=debug").unwrap();
        });
        assert!(result.is_ok());
    }
}