//! Integration test demonstrating bank and user module interaction
//! 
//! This test creates a user and then uses BankService to create an account,
//! demonstrating the modular architecture in action.

mod common;

use rust_kickstart::{UserService, BankService, BankError, CreateUser};
use common::TestContext;
use sqlx::postgres::PgPoolOptions;

/// Helper function to create services with test database
async fn create_test_services() -> (UserService, BankService, TestContext) {
    let ctx = TestContext::new().await;
    
    // Create a new pool with the same configuration as TestContext
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let schema_name = ctx.schema_name.clone();
    let test_pool = PgPoolOptions::new()
        .max_connections(5)
        .after_connect(move |conn, _meta| {
            let schema = schema_name.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}, public"))
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
        .expect("Failed to create test pool");
    
    let user_service = UserService::new(test_pool);
    let bank_service = BankService::new(user_service.clone());
    
    (user_service, bank_service, ctx)
}

#[tokio::test]
async fn test_bank_user_integration() {
    // Setup test services with isolated database
    let (user_service, bank_service, ctx) = create_test_services().await;
    
    // Test data
    let create_user_data = CreateUser {
        name: "John Doe".to_string(),
        age: 30,
    };
    
    // Step 1: Create a user through UserService
    let created_user = user_service
        .create_user(create_user_data)
        .await
        .expect("Failed to create user");
    
    // Step 2: Create bank account for the user
    let account_result = bank_service
        .create_account(created_user.id, 10.00)
        .await
        .expect("Failed to create bank account");
    
    // Step 3: Get account info (demonstrates UserService usage by BankService)
    let account_info = bank_service
        .get_account_info(created_user.id)
        .await
        .expect("Failed to get account info");
    
    // Step 4: Get account holder name
    let holder_name = bank_service
        .get_account_holder_name(created_user.id)
        .await
        .expect("Failed to get account holder name");
    
    // Assertions - verify user creation
    assert_eq!(created_user.name, "John Doe", "User name should match input");
    assert_eq!(created_user.age, 30, "User age should match input");
    
    // Assertions - verify bank account creation
    assert!(
        account_result.contains(&created_user.id.to_string()),
        "Account result should contain user ID"
    );
    assert!(
        account_result.contains("10.00"),
        "Account result should contain the specified balance"
    );
    
    // Assertions - verify account info
    assert_eq!(account_info.user_name, "John Doe", "Account info should have correct user name");
    assert_eq!(account_info.user_age, 30, "Account info should have correct user age");
    assert_eq!(account_info.account_balance, 1000.0, "Account should have mock balance");
    assert_eq!(account_info.account_status, "Active", "Account should be active");
    
    // Assertions - verify account holder name
    assert_eq!(holder_name, "John Doe", "Account holder name should match user name");
    
    // Cleanup
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_bank_service_with_nonexistent_user() {
    // Setup test services with isolated database
    let (_user_service, bank_service, ctx) = create_test_services().await;
    
    // Try to create account for non-existent user
    let result = bank_service.create_account(99999, 10.00).await;
    
    // Should fail because user doesn't exist
    assert!(result.is_err(), "Creating account for non-existent user should fail");
    
    match result {
        Err(BankError::UserNotFound) => {
            // Expected behavior - test passes
        }
        Err(other_error) => panic!("Expected UserNotFound error, got: {:?}", other_error),
        Ok(_) => panic!("Expected error but operation succeeded"),
    }
    
    // Cleanup
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_bank_service_update_account_holder() {
    // Setup test services with isolated database
    let (user_service, bank_service, ctx) = create_test_services().await;
    
    // Create a user
    let create_user_data = CreateUser {
        name: "Jane Smith".to_string(),
        age: 25,
    };
    
    let created_user = user_service
        .create_user(create_user_data)
        .await
        .expect("Failed to create user");
    
    // Update account holder name through BankService
    let updated_user = bank_service
        .update_account_holder(created_user.id, Some("Jane Doe".to_string()))
        .await
        .expect("Failed to update account holder");
    
    // Verify the update through BankService
    assert_eq!(updated_user.name, "Jane Doe", "Updated user name should be 'Jane Doe'");
    assert_eq!(updated_user.age, 25, "User age should remain unchanged");
    assert_eq!(updated_user.id, created_user.id, "User ID should remain the same");
    
    // Verify through UserService as well (ensures consistency)
    let user_from_service = user_service
        .get_user_by_id(created_user.id)
        .await
        .expect("Failed to get updated user");
    
    assert_eq!(
        user_from_service.name, 
        "Jane Doe",
        "UserService should reflect the update made through BankService"
    );
    
    // Cleanup
    ctx.cleanup().await;
}