//! # Rust Kickstart API Server
//!
//! Main entry point for the Rust Kickstart API server.
//! Configures logging and starts the HTTP server.

use rust_kickstart::{create_app, AppConfig};
use rust_kickstart::config::tracing as tracing_config;

#[tokio::main]
async fn main() {
    eprintln!("🚀 Starting Rust Kickstart application...");
    
    // Load environment variables from .env file first
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("⚠️  Warning: Could not load .env file: {}", e);
        eprintln!("💡 Make sure you have a .env file in the project root");
    } else {
        eprintln!("✅ Environment variables loaded from .env file");
    }
    
    eprintln!("🔧 OTEL_EXPORTER_OTLP_ENDPOINT: {:?}", std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT"));
    
    // Initialize enhanced tracing configuration
    if let Err(e) = tracing_config::init() {
        eprintln!("Failed to initialize tracing: {}", e);
        std::process::exit(1);
    }

    tracing::info!("Starting Rust Kickstart API server");

    // Load configuration for server settings
    let config = AppConfig::load();
    tracing::info!(
        "Loaded configuration for environment: {}",
        config.environment
    );

    let app = create_app().await;

    // Use configuration for server settings
    let addr = config.server.address();

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    let local_addr = listener.local_addr().expect("Failed to get local address");

    // Print clickable links
    #[allow(clippy::print_stdout)]
    {
        let display_addr = match config.environment.as_str() {
            "development" => format!("localhost:{}", local_addr.port()),
            _ => local_addr.to_string(),
        };

        println!("\n🚀 Server running!");
        println!("📍 Local:    http://{display_addr}");
        println!("📖 Docs:     http://{display_addr}/swagger-ui");
        println!("🔗 API:      http://{display_addr}/api-docs/openapi.json");
        println!("\nPress Ctrl+C to stop\n");
    }

    tracing::info!("Server listening on {}", local_addr);

    // Setup graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        tracing::info!("Shutdown signal received, starting graceful shutdown...");
        
        // Shutdown OpenTelemetry to flush remaining telemetry data
        tracing_config::shutdown();
        tracing::info!("OpenTelemetry shutdown completed");
    };

    // Run server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .expect("Server failed to start");

    tracing::info!("Server shutdown completed");
}
