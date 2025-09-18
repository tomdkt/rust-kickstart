//! User update service
//! 
//! Handles the business logic for updating existing users with proper validation.

use tracing::{info, warn};

use crate::user::domain::{User, UpdateUser, UserError};
use crate::user::validation::validate_update_user;
use crate::user::repository::UserRepository;

/// Service for updating users
pub struct UpdateUserService;

impl UpdateUserService {
    /// Updates an existing user with validation
    pub(in crate::user) async fn update_user(
        repository: &UserRepository,
        id: i32,
        user_data: UpdateUser,
    ) -> Result<User, UserError> {
        info!(user_id = id, ?user_data, "UpdateUserService: Updating user");

        // Validate input
        if let Err(validation_errors) = validate_update_user(&user_data) {
            warn!(?validation_errors, "UpdateUserService: Validation failed for update user");
            return Err(UserError::ValidationError(validation_errors));
        }

        // First check if user exists
        let Some(existing_user) = repository.find_by_id(id).await? else {
            warn!(user_id = id, "UpdateUserService: User not found for update");
            return Err(UserError::NotFound);
        };

        // Delegate to repository
        repository.update(id, &user_data, &existing_user).await
    }
}