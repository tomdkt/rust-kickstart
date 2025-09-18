//! Integration test demonstrating bank and user module interaction
//! 
//! This test creates a user and then uses BankService to create an account,
//! demonstrating the modular architecture in action.

mod common;

use rust_kickstart::{BankError, CreateUser, UserService, BankService};
use common::TestContext;

/// Creates a UserService instance using the test database pool
fn create_user_service(ctx: &TestContext) -> UserService {
    UserService::new(ctx.get_test_pool().clone())
}

/// Creates a BankService instance using the test database pool
fn create_bank_service(ctx: &TestContext) -> BankService {
    let user_service = create_user_service(ctx);
    BankService::new(user_service)
}

/// Creates both UserService and BankService instances for convenience
fn create_services(ctx: &TestContext) -> (UserService, BankService) {
    let user_service = create_user_service(ctx);
    let bank_service = BankService::new(user_service.clone());
    (user_service, bank_service)
}

#[tokio::test]
async fn test_bank_user_integration() {
    // Arrange
    let ctx = TestContext::new().await;
    let (user_service, bank_service) = create_services(&ctx);
    let create_user_data = CreateUser {
        name: "John Doe".to_string(),
        age: 30,
    };
    let initial_balance = 10.00;

    // Act
    let created_user = user_service
        .create_user(create_user_data)
        .await
        .expect("Failed to create user");
    
    let account_result = bank_service
        .create_account(created_user.id, initial_balance)
        .await
        .expect("Failed to create bank account");
    
    let account_info = bank_service
        .get_account_info(created_user.id)
        .await
        .expect("Failed to get account info");
    
    let holder_name = bank_service
        .get_account_holder_name(created_user.id)
        .await
        .expect("Failed to get account holder name");

    // Assert
    // Verify user creation
    assert_eq!(created_user.name, "John Doe", "User name should match input");
    assert_eq!(created_user.age, 30, "User age should match input");
    
    // Verify bank account creation
    assert!(
        account_result.contains(&created_user.id.to_string()),
        "Account result should contain user ID"
    );
    assert!(
        account_result.contains("10.00"),
        "Account result should contain the specified balance"
    );
    
    // Verify account info
    assert_eq!(account_info.user_name, "John Doe", "Account info should have correct user name");
    assert_eq!(account_info.user_age, 30, "Account info should have correct user age");
    assert_eq!(account_info.account_balance, 1000.0, "Account should have mock balance");
    assert_eq!(account_info.account_status, "Active", "Account should be active");
    
    // Verify account holder name
    assert_eq!(holder_name, "John Doe", "Account holder name should match user name");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_bank_service_with_nonexistent_user() {
    // Arrange
    let ctx = TestContext::new().await;
    let bank_service = create_bank_service(&ctx);
    let nonexistent_user_id = 99999;
    let initial_balance = 10.00;

    // Act
    let result = bank_service.create_account(nonexistent_user_id, initial_balance).await;

    // Assert
    assert!(result.is_err(), "Creating account for non-existent user should fail");
    
    match result {
        Err(BankError::UserNotFound) => {
            // Expected behavior - test passes
        }
        Err(other_error) => panic!("Expected UserNotFound error, got: {:?}", other_error),
        Ok(_) => panic!("Expected error but operation succeeded"),
    }
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_bank_service_update_account_holder() {
    // Arrange
    let ctx = TestContext::new().await;
    let (user_service, bank_service) = create_services(&ctx);
    let create_user_data = CreateUser {
        name: "Jane Smith".to_string(),
        age: 25,
    };
    let new_name = "Jane Doe".to_string();

    // Act
    let created_user = user_service
        .create_user(create_user_data)
        .await
        .expect("Failed to create user");
    
    let updated_user = bank_service
        .update_account_holder(created_user.id, Some(new_name.clone()))
        .await
        .expect("Failed to update account holder");
    
    let user_from_service = user_service
        .get_user_by_id(created_user.id)
        .await
        .expect("Failed to get updated user");

    // Assert
    // Verify the update through BankService
    assert_eq!(updated_user.name, "Jane Doe", "Updated user name should be 'Jane Doe'");
    assert_eq!(updated_user.age, 25, "User age should remain unchanged");
    assert_eq!(updated_user.id, created_user.id, "User ID should remain the same");
    
    // Verify through UserService as well (ensures consistency)
    assert_eq!(
        user_from_service.name, 
        "Jane Doe",
        "UserService should reflect the update made through BankService"
    );
    
    ctx.cleanup().await;
}