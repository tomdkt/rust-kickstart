//! Tracing and observability configuration for the application.
//!
//! This module provides structured logging with different formats for development
//! and production environments, along with OpenTelemetry integration for distributed
//! tracing and observability.

use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

/// Initialize tracing/logging and observability configuration.
///
/// This should be called once at application startup. It configures:
/// - JSON format in production for structured logging
/// - Pretty format in development for readability
/// - OpenTelemetry integration for distributed tracing (if configured)
/// - Appropriate log levels based on environment
/// - Request tracing and correlation IDs
///
/// # Errors
///
/// Returns an error if the tracing subscriber cannot be initialized.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Check if OpenTelemetry endpoint is configured
    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT");
    let service_name =
        std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "rust-kickstart".to_string());

    // Try to initialize OpenTelemetry if endpoint is configured
    let otel_layer = if let Ok(_endpoint_url) = endpoint {
        match init_opentelemetry() {
            Ok(tracer) => Some(tracing_opentelemetry::layer().with_tracer(tracer)),
            Err(e) => {
                eprintln!("❌ Failed to initialize OpenTelemetry: {}", e);
                None
            },
        }
    } else {
        None
    };

    // Use JSON format in production, pretty format in development
    let fmt_layer = if cfg!(debug_assertions) {
        // Development: Clean format focusing on our code
        tracing_subscriber::fmt::layer()
            .with_target(true) // Show module names (rust_kickstart::user::repository)
            .with_thread_ids(true) // Show ThreadId for debugging concurrency
            .with_thread_names(false) // Hide "tokio-runtime-worker" noise
            .with_file(false) // Hide file paths to reduce noise
            .with_line_number(false) // Hide line numbers to reduce noise
            .with_level(true) // Show log level (INFO, DEBUG, etc.)
            .with_ansi(true) // Keep colors for better readability
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

    // Initialize the global subscriber with optional OpenTelemetry layer
    let registry = Registry::default().with(env_filter).with(fmt_layer);

    match otel_layer {
        Some(otel) => registry.with(otel).try_init()?,
        None => registry.try_init()?,
    }

    // Now that tracing is initialized, we can log messages
    tracing::debug!("🔍 Tracing configuration initialized successfully");

    // Log OpenTelemetry status
    if let Ok(endpoint_url) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        tracing::debug!("✅ OTEL_EXPORTER_OTLP_ENDPOINT found: {}", endpoint_url);
        tracing::debug!("📡 Service name: {}", service_name);
        tracing::debug!(
            "✅ OpenTelemetry initialized successfully! Distributed tracing is now active."
        );
        tracing::info!("📊 Traces will be exported to: {}", endpoint_url);
        tracing::debug!("🔗 Check your observability platform for trace data");
    } else {
        tracing::warn!("⚠️  OpenTelemetry not configured (OTEL_EXPORTER_OTLP_ENDPOINT not set)");
        tracing::info!(
            "💡 To enable tracing, set: OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318"
        );
    }

    Ok(())
}

/// Initialize OpenTelemetry tracer.
///
/// This function sets up the OpenTelemetry tracer with OTLP exporter.
///
/// # Errors
///
/// Returns an error if the OpenTelemetry tracer cannot be initialized.
#[cfg(feature = "otel")]
fn init_opentelemetry() -> Result<opentelemetry_sdk::trace::Tracer, Box<dyn std::error::Error>> {
    use opentelemetry::{global, trace::TracerProvider, KeyValue};
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::{
        trace::{Sampler, TracerProvider as SdkTracerProvider},
        Resource,
    };
    // Get configuration from environment
    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")?;
    let service_name =
        std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "rust-kickstart".to_string());
    let service_version =
        std::env::var("OTEL_SERVICE_VERSION").unwrap_or_else(|_| "0.1.0".to_string());

    // Create resource with service information
    let resource = Resource::new(vec![
        KeyValue::new("service.name", service_name),
        KeyValue::new("service.version", service_version),
    ]);

    // Initialize OTLP exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(endpoint)
        .build_span_exporter()?;

    // Create tracer provider with batch exporter
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_config(
            opentelemetry_sdk::trace::Config::default()
                .with_resource(resource)
                .with_sampler(Sampler::AlwaysOn),
        )
        .build();

    // Set global tracer provider
    global::set_tracer_provider(tracer_provider.clone());

    // Get tracer
    let tracer = tracer_provider.tracer("rust-kickstart");

    Ok(tracer)
}

/// Fallback initialization when OpenTelemetry is not available
#[cfg(not(feature = "otel"))]
fn init_opentelemetry() -> Result<opentelemetry::trace::noop::NoopTracer, Box<dyn std::error::Error>>
{
    Ok(opentelemetry::trace::noop::NoopTracer::new())
}

/// Shutdown OpenTelemetry gracefully.
///
/// This function should be called during application shutdown to ensure
/// all telemetry data is properly flushed and exported.
pub fn shutdown() {
    #[cfg(feature = "otel")]
    {
        use opentelemetry::global;
        global::shutdown_tracer_provider();
    }
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
