//! # Rust Kickstart API Server
//!
//! Main entry point for the Rust Kickstart API server.
//! Configures logging and starts the HTTP server.

use rust_kickstart::{create_app, AppConfig};
use rust_kickstart::config::tracing as tracing_config;

#[tokio::main]
async fn main() {
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
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
