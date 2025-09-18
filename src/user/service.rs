//! User service - business logic layer
//! 
//! This is the main interface that other modules should use to interact with user functionality.
//! It encapsulates business logic and coordinates between domain validation and repository operations.

use sqlx::PgPool;
use tracing::{info, warn};

use super::domain::{User, CreateUser, UpdateUser, UserError, validate_create_user, validate_update_user, ApiResponse};
use super::repository::UserRepository;

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
        info!(?user_data, "UserService: Creating new user");

        // Validate input
        if let Err(validation_errors) = validate_create_user(&user_data) {
            warn!(?validation_errors, "UserService: Validation failed for create user");
            return Err(UserError::ValidationError(validation_errors));
        }

        // Delegate to repository
        self.repository.create(&user_data).await
    }

    /// Retrieves all users
    pub async fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        info!("UserService: Fetching all users");
        self.repository.find_all().await
    }

    /// Retrieves a specific user by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<User, UserError> {
        info!(user_id = id, "UserService: Fetching user by ID");

        match self.repository.find_by_id(id).await? {
            Some(user) => Ok(user),
            None => {
                warn!(user_id = id, "UserService: User not found");
                Err(UserError::NotFound)
            }
        }
    }

    /// Updates an existing user with validation
    pub async fn update_user(&self, id: i32, user_data: UpdateUser) -> Result<User, UserError> {
        info!(user_id = id, ?user_data, "UserService: Updating user");

        // Validate input
        if let Err(validation_errors) = validate_update_user(&user_data) {
            warn!(?validation_errors, "UserService: Validation failed for update user");
            return Err(UserError::ValidationError(validation_errors));
        }

        // First check if user exists
        let existing_user = match self.repository.find_by_id(id).await? {
            Some(user) => user,
            None => {
                warn!(user_id = id, "UserService: User not found for update");
                return Err(UserError::NotFound);
            }
        };

        // Delegate to repository
        self.repository.update(id, &user_data, &existing_user).await
    }

    /// Deletes a user
    pub async fn delete_user(&self, id: i32) -> Result<ApiResponse, UserError> {
        info!(user_id = id, "UserService: Deleting user");

        let deleted = self.repository.delete(id).await?;
        if deleted {
            Ok(ApiResponse {
                message: format!("User with id {id} deleted successfully"),
            })
        } else {
            warn!(user_id = id, "UserService: User not found for deletion");
            Err(UserError::NotFound)
        }
    }

    /// Checks if a user exists (utility method for other modules)
    pub async fn user_exists(&self, id: i32) -> Result<bool, UserError> {
        info!(user_id = id, "UserService: Checking if user exists");
        
        match self.repository.find_by_id(id).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Gets user name by ID (utility method for other modules)
    pub async fn get_user_name(&self, id: i32) -> Result<String, UserError> {
        info!(user_id = id, "UserService: Getting user name");
        
        match self.repository.find_by_id(id).await? {
            Some(user) => Ok(user.name),
            None => Err(UserError::NotFound),
        }
    }
}