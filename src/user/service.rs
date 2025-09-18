//! User service - business logic layer
//! 
//! This is the main interface that other modules should use to interact with user functionality.
//! It encapsulates business logic and coordinates between domain validation and repository operations.
//! 
//! The service is now modularized with separate service modules for each operation type,
//! improving maintainability and following Rust best practices.

use sqlx::PgPool;

use super::domain::{User, CreateUser, UpdateUser, UserError, ApiResponse};
use super::repository::UserRepository;
use super::services::{
    CreateUserService, ReadUserService, UpdateUserService, 
    DeleteUserService, UserUtilsService
};

/// User service that handles business logic and coordinates operations
#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// Creates a new UserService instance
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: UserRepository::new(pool),
        }
    }

    /// Creates a new user with validation
    pub async fn create_user(&self, user_data: CreateUser) -> Result<User, UserError> {
        CreateUserService::create_user(&self.repository, user_data).await
    }

    /// Retrieves all users
    pub async fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        ReadUserService::get_all_users(&self.repository).await
    }

    /// Retrieves a specific user by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<User, UserError> {
        ReadUserService::get_user_by_id(&self.repository, id).await
    }

    /// Updates an existing user with validation
    pub async fn update_user(&self, id: i32, user_data: UpdateUser) -> Result<User, UserError> {
        UpdateUserService::update_user(&self.repository, id, user_data).await
    }

    /// Deletes a user
    pub async fn delete_user(&self, id: i32) -> Result<ApiResponse, UserError> {
        DeleteUserService::delete_user(&self.repository, id).await
    }

    /// Checks if a user exists (utility method for other modules)
    pub async fn user_exists(&self, id: i32) -> Result<bool, UserError> {
        UserUtilsService::user_exists(&self.repository, id).await
    }

    /// Gets user name by ID (utility method for other modules)
    pub async fn get_user_name(&self, id: i32) -> Result<String, UserError> {
        UserUtilsService::get_user_name(&self.repository, id).await
    }
}