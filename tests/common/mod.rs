use rust_kickstart::create_app_with_pool;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use uuid::Uuid;
use std::sync::Once;

static INIT: Once = Once::new();

#[allow(dead_code)]
pub struct TestContext {
    pub app: axum::Router,
    pub schema_name: String,
    pub pool: PgPool,
    pub test_pool: PgPool,
}

impl TestContext {
    pub async fn new() -> Self {
        // Initialize tracing subscriber once
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .init();
        });

        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Generate unique schema name using UUID v7
        let schema_name = format!("test_{}", Uuid::now_v7().simple());

        info!("[TEST_SETUP] Creating test schema: {}", schema_name);

        // Connect to database
        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        // Create the test schema
        sqlx::query(&format!("CREATE SCHEMA {schema_name}"))
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
                    sqlx::query(&format!("SET search_path TO {schema}, public"))
                        .execute(conn)
                        .await?;
                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("Failed to create test pool");

        // Create app with the test pool
        let app = create_app_with_pool(test_pool.clone());

        info!("[TEST_SETUP] ✅ Test schema {} created and ready", schema_name);

        Self {
            app,
            schema_name,
            pool: admin_pool, // Keep admin pool for cleanup
            test_pool,
        }
    }

    async fn run_migrations(pool: &PgPool, schema_name: &str) {
        info!("[TEST_SETUP] Running migrations in schema: {}", schema_name);

        // Set search_path to the test schema for migrations
        sqlx::query(&format!("SET search_path TO {schema_name}, public"))
            .execute(pool)
            .await
            .expect("Failed to set search_path for migrations");

        // Run migrations from the ./migrations folder - single source of truth
        // This ensures tests use the same schema definition as production
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .expect("Failed to run migrations in test schema");

        info!("[TEST_SETUP] ✅ Migrations completed for schema: {}", schema_name);
    }

    /// Returns a reference to the test database pool for service creation
    #[allow(dead_code)]
    pub fn get_test_pool(&self) -> &PgPool {
        &self.test_pool
    }

    #[allow(clippy::print_stderr)]
    pub async fn cleanup(&self) {
        info!("[TEST_CLEANUP] Starting cleanup for test schema: {}", self.schema_name);

        if let Err(e) = sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema_name))
            .execute(&self.pool)
            .await
        {
            eprintln!("[TEST_CLEANUP] ❌ Failed to drop test schema {}: {}", self.schema_name, e);
        } else {
            info!("[TEST_CLEANUP] ✅ Test schema {} dropped successfully", self.schema_name);
        }
    }
}

#[allow(clippy::print_stderr)]
impl Drop for TestContext {
    fn drop(&mut self) {
        // Try to cleanup synchronously if we're in a tokio context
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let schema_name = self.schema_name.clone();
            let pool = self.pool.clone();

            handle.spawn(async move {
                info!("[TEST_CLEANUP] Starting cleanup for test schema: {}", schema_name);

                if let Err(e) = sqlx::query(&format!("DROP SCHEMA {schema_name} CASCADE"))
                    .execute(&pool)
                    .await
                {
                    eprintln!("[TEST_CLEANUP] ❌ Failed to drop test schema {schema_name}: {e}");
                } else {
                    info!("[TEST_CLEANUP] ✅ Test schema {} dropped successfully", schema_name);
                }
            });
        }
    }
}
