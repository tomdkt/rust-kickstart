//! Application configuration module

use std::env;
use super::{DatabaseConfig, ServerConfig};

/// Main application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    /// Server configuration
    pub server: ServerConfig,
    /// Environment (development, production, etc.)
    pub environment: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn load() -> Self {
        // Load .env file if it exists (for development)
        dotenvy::dotenv().ok();

        Self {
            database: DatabaseConfig::load(),
            server: ServerConfig::load(),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}