//! User utility services
//! 
//! Contains utility methods that other modules might need for user operations.

use tracing::info;

use crate::user::domain::{UserError};
use crate::user::repository::UserRepository;

/// Service for user utility operations
pub struct UserUtilsService;

impl UserUtilsService {
    /// Checks if a user exists (utility method for other modules)
    pub(in crate::user) async fn user_exists(repository: &UserRepository, id: i32) -> Result<bool, UserError> {
        info!(user_id = id, "UserUtilsService: Checking if user exists");
        
        match repository.find_by_id(id).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Gets user name by ID (utility method for other modules)
    pub(in crate::user) async fn get_user_name(repository: &UserRepository, id: i32) -> Result<String, UserError> {
        info!(user_id = id, "UserUtilsService: Getting user name");
        
        match repository.find_by_id(id).await? {
            Some(user) => Ok(user.name),
            None => Err(UserError::NotFound),
        }
    }
}