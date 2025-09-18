use rust_kickstart::create_app;
use tracing::info;

pub struct TestContext {
    pub app: axum::Router,
}

impl TestContext {
    pub async fn new() -> Self {
        // Use the existing database connection from .env
        let app = create_app().await;

        Self { app }
    }

    pub async fn cleanup_database() {
        info!("Cleaning up test database");
        
        // Connect to database and clean up
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database for cleanup");

        // Clean all tables between tests
        sqlx::query!("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await
            .expect("Failed to cleanup test database");
            
        info!("Test database cleaned successfully");
    }
}