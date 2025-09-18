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
    #[must_use] pub fn load() -> Self {
        Self {
            host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_owned()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_owned())
                .parse()
                .unwrap_or(3000),
        }
    }

    /// Get server address as string
    #[must_use] pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}