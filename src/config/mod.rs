//! Configuration module for the application
//!
//! Organized configuration using environment variables and tracing setup.

mod app;
mod database;
mod server;
pub mod tracing;

// Re-export all configuration types
pub use app::AppConfig;
pub use database::DatabaseConfig;
pub use server::ServerConfig;