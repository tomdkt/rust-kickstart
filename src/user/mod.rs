//! User module
//! 
//! This module provides user management functionality with proper separation of concerns.
//! External modules can only access the UserService, not the repository or controller directly.

pub mod domain;
pub mod repository;
pub mod service;
pub mod controller;

// Public exports - only UserService is exposed to other modules
pub use service::UserService;

// Re-export domain types that other modules might need
pub use domain::{User, CreateUser, UpdateUser};

// Export controller for OpenAPI documentation (but discourage direct use)
pub use controller::*;