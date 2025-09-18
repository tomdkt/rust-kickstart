use rust_kickstart::create_app_with_pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;


pub struct TestContext {
    pub app: axum::Router,
    pub schema_name: String,
    pub pool: PgPool,
}

impl TestContext {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        // Generate unique schema name using UUID v7
        let schema_name = format!("test_{}", Uuid::now_v7().simple());
        
        info!("Creating test schema: {}", schema_name);
        
        // Connect to database
        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        // Create the test schema
        sqlx::query(&format!("CREATE SCHEMA {}", schema_name))
            .execute(&admin_pool)
            .await
            .expect("Failed to create test schema");

        // Run migrations in the test schema
        Self::run_migrations(&admin_pool, &schema_name).await;

        // Create a new pool with search_path set to the test schema
        let schema_for_closure = schema_name.clone();
        let test_pool = PgPoolOptions::new()
            .max_connections(5)
            .after_connect(move |conn, _meta| {
                let schema = schema_for_closure.clone();
                Box::pin(async move {
                    sqlx::query(&format!("SET search_path TO {}, public", schema))
                        .execute(conn)
                        .await?;
                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("Failed to create test pool");

        // Create app with the test pool
        let app = create_app_with_pool(test_pool.clone()).await;

        info!("Test schema {} created and ready", schema_name);

        Self {
            app,
            schema_name,
            pool: admin_pool, // Keep admin pool for cleanup
        }
    }

    async fn run_migrations(pool: &PgPool, schema_name: &str) {
        info!("Running migrations in schema: {}", schema_name);
        
        // Create users table in the test schema
        sqlx::query(&format!(
            "CREATE TABLE {}.users (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                age INT NOT NULL
            )", schema_name
        ))
        .execute(pool)
        .await
        .expect("Failed to create users table in test schema");
        
        info!("Migrations completed for schema: {}", schema_name);
    }


}

impl TestContext {
    pub async fn cleanup(&self) {
        info!("Dropping test schema: {}", self.schema_name);
        
        if let Err(e) = sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema_name))
            .execute(&self.pool)
            .await
        {
            eprintln!("Failed to drop test schema {}: {}", self.schema_name, e);
        } else {
            info!("Test schema {} dropped successfully", self.schema_name);
        }
    }
}



impl Drop for TestContext {
    fn drop(&mut self) {
        // Try to cleanup synchronously if we're in a tokio context
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let schema_name = self.schema_name.clone();
            let pool = self.pool.clone();
            
            handle.spawn(async move {
                info!("Dropping test schema: {}", schema_name);
                
                if let Err(e) = sqlx::query(&format!("DROP SCHEMA {} CASCADE", schema_name))
                    .execute(&pool)
                    .await
                {
                    eprintln!("Failed to drop test schema {}: {}", schema_name, e);
                } else {
                    info!("Test schema {} dropped successfully", schema_name);
                }
            });
        }
    }
}