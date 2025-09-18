//! User deletion service
//! 
//! Handles the business logic for deleting users.

use tracing::{info, warn};

use crate::user::domain::{UserError, ApiResponse};
use crate::user::repository::UserRepository;

/// Service for deleting users
pub struct DeleteUserService;

impl DeleteUserService {
    /// Deletes a user
    pub(in crate::user) async fn delete_user(
        repository: &UserRepository,
        id: i32,
    ) -> Result<ApiResponse, UserError> {
        info!(user_id = id, "DeleteUserService: Deleting user");

        let deleted = repository.delete(id).await?;
        if deleted {
            Ok(ApiResponse {
                message: format!("User with id {id} deleted successfully"),
            })
        } else {
            warn!(user_id = id, "DeleteUserService: User not found for deletion");
            Err(UserError::NotFound)
        }
    }
}