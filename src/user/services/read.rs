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
    #[tracing::instrument(skip(repository))]
    pub(in crate::user) async fn get_all_users(repository: &UserRepository) -> Result<Vec<User>, UserError> {
        info!("ReadUserService: Fetching all users");
        let users = repository.find_all().await?;
        info!(user_count = users.len(), "ReadUserService: Successfully fetched users");
        Ok(users)
    }

    /// Retrieves a specific user by ID
    #[tracing::instrument(skip(repository), fields(user_id = id))]
    pub(in crate::user) async fn get_user_by_id(
        repository: &UserRepository,
        id: i32,
    ) -> Result<User, UserError> {
        info!(user_id = id, "ReadUserService: Fetching user by ID");

        if let Some(user) = repository.find_by_id(id).await? { 
            info!(user_id = id, user_name = %user.name, "ReadUserService: User found successfully");
            Ok(user) 
        } else {
            warn!(user_id = id, "ReadUserService: User not found");
            Err(UserError::NotFound)
        }
    }
}