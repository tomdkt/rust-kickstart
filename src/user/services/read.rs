//! User reading service
//! 
//! Handles the business logic for reading/retrieving user data.

use tracing::{info, warn};

use crate::user::domain::{User, UserError, PaginationParams, PaginatedUsersResponse};
use crate::user::repository::UserRepository;
use crate::pagination::PaginationToken;

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

    /// Retrieves users with pagination using cursor tokens
    #[tracing::instrument(skip(repository), fields(next_token = params.next_token.as_deref(), limit = params.limit))]
    pub(in crate::user) async fn get_users_paginated(
        repository: &UserRepository,
        params: PaginationParams,
    ) -> Result<PaginatedUsersResponse, UserError> {
        // Default limit is 200, max is 200
        let limit = params.limit.unwrap_or(200).min(200).max(1);
        
        info!(next_token = params.next_token.as_deref(), limit = limit, "ReadUserService: Fetching paginated users");

        // Decode the pagination token if provided
        let cursor = match params.next_token {
            Some(token) => {
                let (last_id, last_timestamp) = PaginationToken::decode(&token)
                    .map_err(|_| UserError::InvalidToken)?;
                Some((last_id, last_timestamp))
            }
            None => None,
        };

        // Fetch one extra record to check if there are more pages
        let users = repository.find_paginated(cursor, limit + 1).await?;
        
        let has_more = users.len() > limit as usize;
        let mut result_users = users;
        
        // Remove the extra record if we have more than the limit
        if has_more {
            result_users.pop();
        }
        
        // Generate next token if there are more pages
        let next_token = if has_more {
            result_users.last().map(|user| {
                PaginationToken::encode(user.id, user.created_at)
                    .unwrap_or_else(|_| String::new())
            }).filter(|token| !token.is_empty())
        } else {
            None
        };
        
        let count = result_users.len();
        
        info!(
            count = count,
            has_more = has_more,
            next_token = next_token.as_deref(),
            "ReadUserService: Successfully fetched paginated users"
        );

        Ok(PaginatedUsersResponse {
            users: result_users,
            next_token,
            has_more,
            count,
        })
    }
}