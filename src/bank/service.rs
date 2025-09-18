//! Bank service - demonstrates inter-module communication
//! 
//! This service shows how the bank module can use `UserService`
//! but cannot directly access `UserRepository`.

use tracing::{info, warn};

use crate::user::{UserService, User};
use crate::user::domain::UserError;

/// Bank service that needs to interact with users
#[derive(Clone)]
pub struct BankService {
    user_service: UserService,
}

impl BankService {
    /// Creates a new `BankService` instance
    #[must_use] pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    /// Creates a bank account for a user (requires user to exist)
    pub async fn create_account(&self, user_id: i32, initial_balance: f64) -> Result<String, BankError> {
        info!(user_id, initial_balance, "BankService: Creating account for user");

        // We can use UserService to check if user exists
        match self.user_service.user_exists(user_id).await {
            Ok(true) => {
                // User exists, create account
                info!(user_id, "BankService: User exists, creating account");
                Ok(format!("Account created for user {user_id} with balance ${initial_balance:.2}"))
            }
            Ok(false) => {
                warn!(user_id, "BankService: User not found, cannot create account");
                Err(BankError::UserNotFound)
            }
            Err(e) => {
                warn!(user_id, error = ?e, "BankService: Error checking user existence");
                Err(BankError::UserServiceError(e))
            }
        }
    }

    /// Gets account information with user details
    pub async fn get_account_info(&self, user_id: i32) -> Result<AccountInfo, BankError> {
        info!(user_id, "BankService: Getting account info for user");

        // We can get user details through UserService
        match self.user_service.get_user_by_id(user_id).await {
            Ok(user) => {
                info!(user_id, user_name = %user.name, "BankService: Found user for account info");
                Ok(AccountInfo {
                    user_id: user.id,
                    user_name: user.name,
                    user_age: user.age,
                    account_balance: 1000.0, // Mock balance
                    account_status: "Active".to_owned(),
                })
            }
            Err(UserError::NotFound) => {
                warn!(user_id, "BankService: User not found for account info");
                Err(BankError::UserNotFound)
            }
            Err(e) => {
                warn!(user_id, error = ?e, "BankService: Error getting user details");
                Err(BankError::UserServiceError(e))
            }
        }
    }

    /// Updates account holder information (delegates to `UserService`)
    pub async fn update_account_holder(&self, user_id: i32, new_name: Option<String>) -> Result<User, BankError> {
        info!(user_id, ?new_name, "BankService: Updating account holder information");

        // We can use UserService to update user information
        let update_data = crate::user::UpdateUser {
            name: new_name,
            age: None,
        };

        match self.user_service.update_user(user_id, update_data).await {
            Ok(user) => {
                info!(user_id, "BankService: Account holder information updated");
                Ok(user)
            }
            Err(UserError::NotFound) => {
                warn!(user_id, "BankService: User not found for update");
                Err(BankError::UserNotFound)
            }
            Err(e) => {
                warn!(user_id, error = ?e, "BankService: Error updating user");
                Err(BankError::UserServiceError(e))
            }
        }
    }

    /// Gets user name for account operations
    pub async fn get_account_holder_name(&self, user_id: i32) -> Result<String, BankError> {
        info!(user_id, "BankService: Getting account holder name");

        match self.user_service.get_user_name(user_id).await {
            Ok(name) => {
                info!(user_id, user_name = %name, "BankService: Got account holder name");
                Ok(name)
            }
            Err(UserError::NotFound) => {
                warn!(user_id, "BankService: User not found");
                Err(BankError::UserNotFound)
            }
            Err(e) => {
                warn!(user_id, error = ?e, "BankService: Error getting user name");
                Err(BankError::UserServiceError(e))
            }
        }
    }
}

/// Account information combining user and bank data
#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// User's unique identifier
    pub user_id: i32,
    /// User's full name
    pub user_name: String,
    /// User's age in years
    pub user_age: i32,
    /// Current account balance
    pub account_balance: f64,
    /// Account status (Active, Inactive, etc.)
    pub account_status: String,
}

/// Bank-specific errors
#[derive(Debug, thiserror::Error)]
pub enum BankError {
    /// User was not found in the system
    #[error("User not found")]
    UserNotFound,
    /// Error from the underlying user service
    #[error("User service error: {0}")]
    UserServiceError(UserError),
    /// Account has insufficient funds for the operation
    #[error("Insufficient funds")]
    InsufficientFunds,
    /// Bank account was not found
    #[error("Account not found")]
    AccountNotFound,
}