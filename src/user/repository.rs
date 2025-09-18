//! User repository - handles database operations
//! 
//! This module is private to the user module and cannot be accessed directly
//! by other modules. All database access must go through UserService.

use sqlx::PgPool;
use tracing::{error, info, warn};

use super::domain::{User, CreateUser, UpdateUser, UserError};

/// User repository for database operations
#[derive(Clone)]
pub(super) struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Creates a new UserRepository instance
    pub(super) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new user in the database
    pub(super) async fn create(&self, user_data: &CreateUser) -> Result<User, UserError> {
        info!(?user_data, "Creating new user in database");

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id, name, age",
            user_data.name.trim(),
            user_data.age
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to create user in database");
            UserError::DatabaseError(e.to_string())
        })?;

        info!(user_id = user.id, "User created successfully in database");
        Ok(user)
    }

    /// Retrieves all users from the database
    pub(super) async fn find_all(&self) -> Result<Vec<User>, UserError> {
        info!("Fetching all users from database");

        let users = sqlx::query_as!(User, "SELECT id, name, age FROM users ORDER BY id")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to fetch users from database");
                UserError::DatabaseError(e.to_string())
            })?;

        info!(count = users.len(), "Users fetched successfully from database");
        Ok(users)
    }

    /// Retrieves a specific user by ID from the database
    pub(super) async fn find_by_id(&self, id: i32) -> Result<Option<User>, UserError> {
        info!(user_id = id, "Fetching user by ID from database");

        let user = sqlx::query_as::<_, User>(include_str!("sql/find_user_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, user_id = id, "Failed to fetch user from database");
                UserError::DatabaseError(e.to_string())
            })?;

        if user.is_some() {
            info!(user_id = id, "User found in database");
        } else {
            warn!(user_id = id, "User not found in database");
        }

        Ok(user)
    }

    /// Updates an existing user in the database
    pub(super) async fn update(&self, id: i32, user_data: &UpdateUser, existing_user: &User) -> Result<User, UserError> {
        info!(user_id = id, ?user_data, "Updating user in database");

        // Use existing values if not provided in update, trim name if provided
        let name = user_data
            .name
            .as_ref()
            .map(|n| n.trim().to_owned())
            .unwrap_or_else(|| existing_user.name.clone());
        let age = user_data.age.unwrap_or(existing_user.age);

        let updated_user = sqlx::query_as!(
            User,
            "UPDATE users SET name = $1, age = $2 WHERE id = $3 RETURNING id, name, age",
            name,
            age,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, user_id = id, "Failed to update user in database");
            UserError::DatabaseError(e.to_string())
        })?;

        info!(user_id = id, "User updated successfully in database");
        Ok(updated_user)
    }

    /// Deletes a user from the database
    pub(super) async fn delete(&self, id: i32) -> Result<bool, UserError> {
        info!(user_id = id, "Deleting user from database");

        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, user_id = id, "Failed to delete user from database");
                UserError::DatabaseError(e.to_string())
            })?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            info!(user_id = id, "User deleted successfully from database");
        } else {
            warn!(user_id = id, "User not found for deletion in database");
        }

        Ok(deleted)
    }
}