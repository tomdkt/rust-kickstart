//! Configuration module for the application
//!
//! Organized configuration using environment variables.

mod app;
mod database;
mod server;

// Re-export all configuration types
pub use app::AppConfig;
pub use database::DatabaseConfig;
pub use server::ServerConfig;