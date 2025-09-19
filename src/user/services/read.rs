//! User reading service
//! 
//! Handles the business logic for reading/retrieving user data.

use tracing::{info, warn};

use crate::user::domain::{User, UserError, PaginationParams, PaginatedUsersResponse};
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

    /// Retrieves users with pagination
    #[tracing::instrument(skip(repository), fields(last_id = params.last_id, limit = params.limit))]
    pub(in crate::user) async fn get_users_paginated(
        repository: &UserRepository,
        params: PaginationParams,
    ) -> Result<PaginatedUsersResponse, UserError> {
        // Default limit is 200, max is 200
        let limit = params.limit.unwrap_or(200).min(200).max(1);
        
        info!(last_id = params.last_id, limit = limit, "ReadUserService: Fetching paginated users");

        // Fetch one extra record to check if there are more pages
        let users = repository.find_paginated(params.last_id, limit + 1).await?;
        
        let has_more = users.len() > limit as usize;
        let mut result_users = users;
        
        // Remove the extra record if we have more than the limit
        if has_more {
            result_users.pop();
        }
        
        let last_id = result_users.last().map(|user| user.id);
        let count = result_users.len();
        
        info!(
            count = count,
            last_id = last_id,
            has_more = has_more,
            "ReadUserService: Successfully fetched paginated users"
        );

        Ok(PaginatedUsersResponse {
            users: result_users,
            last_id,
            has_more,
            count,
        })
    }
}