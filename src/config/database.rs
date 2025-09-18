//! Database configuration module

use std::env;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of database connections
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Load database configuration from environment variables
    #[must_use] pub fn load() -> Self {
        Self {
            url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_owned())
                .parse()
                .unwrap_or(5),
        }
    }
}