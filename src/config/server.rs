//! Server configuration module

use std::env;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
}

impl ServerConfig {
    /// Load server configuration from environment variables
    pub fn load() -> Self {
        Self {
            host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        }
    }

    /// Get server address as string
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}