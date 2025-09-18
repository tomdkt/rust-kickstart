//! # Rust Kickstart API Server
//!
//! Main entry point for the Rust Kickstart API server.
//! Configures logging and starts the HTTP server.

use rust_kickstart::{create_app, AppConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_kickstart=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Kickstart API server");

    // Load configuration for server settings
    let config = AppConfig::load();
    tracing::info!("Loaded configuration for environment: {}", config.environment);

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
        println!("\n🚀 Server running!");
        println!("📍 Local:    http://{local_addr}");
        println!("📖 Docs:     http://{local_addr}/swagger-ui");
        println!("🔗 API:      http://{local_addr}/api-docs/openapi.json");
        println!("\nPress Ctrl+C to stop\n");
    }

    tracing::info!("Server listening on {}", local_addr);
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
