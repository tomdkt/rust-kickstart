//! User creation service
//! 
//! Handles the business logic for creating new users with proper validation.

use tracing::{info, warn};

use crate::user::domain::{User, CreateUser, UserError};
use crate::user::validation::validate_create_user;
use crate::user::repository::UserRepository;

/// Service for creating users
pub struct CreateUserService;

impl CreateUserService {
    /// Creates a new user with validation
    pub(in crate::user) async fn create_user(
        repository: &UserRepository,
        user_data: CreateUser,
    ) -> Result<User, UserError> {
        info!(?user_data, "CreateUserService: Creating new user");

        // Validate input
        if let Err(validation_errors) = validate_create_user(&user_data) {
            warn!(?validation_errors, "CreateUserService: Validation failed for create user");
            return Err(UserError::ValidationError(validation_errors));
        }

        // Delegate to repository
        repository.create(&user_data).await
    }
}