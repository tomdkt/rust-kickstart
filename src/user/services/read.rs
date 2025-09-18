//! User reading service
//! 
//! Handles the business logic for reading/retrieving user data.

use tracing::{info, warn};

use crate::user::domain::{User, UserError};
use crate::user::repository::UserRepository;

/// Service for reading users
pub struct ReadUserService;

impl ReadUserService {
    /// Retrieves all users
    pub(in crate::user) async fn get_all_users(repository: &UserRepository) -> Result<Vec<User>, UserError> {
        info!("ReadUserService: Fetching all users");
        repository.find_all().await
    }

    /// Retrieves a specific user by ID
    pub(in crate::user) async fn get_user_by_id(
        repository: &UserRepository,
        id: i32,
    ) -> Result<User, UserError> {
        info!(user_id = id, "ReadUserService: Fetching user by ID");

        match repository.find_by_id(id).await? {
            Some(user) => Ok(user),
            None => {
                warn!(user_id = id, "ReadUserService: User not found");
                Err(UserError::NotFound)
            }
        }
    }
}