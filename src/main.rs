use rust_kickstart::create_app;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_kickstart=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Kickstart API server");

    let app = create_app().await;
    
    // Allow configuration via environment variables
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    let local_addr = listener.local_addr().unwrap();
    
    // Print clickable links
    println!("\n🚀 Server running!");
    println!("📍 Local:    http://{}", local_addr);
    println!("📖 Docs:     http://{}/swagger-ui", local_addr);
    println!("🔗 API:      http://{}/api-docs/openapi.json", local_addr);
    println!("\nPress Ctrl+C to stop\n");
    
    tracing::info!("Server listening on {}", local_addr);
    axum::serve(listener, app).await.unwrap();
}