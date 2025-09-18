//! Example demonstrating the modular architecture
//! 
//! This example shows how the bank module can use UserService
//! but cannot access UserRepository directly.

use rust_kickstart::user::UserService;
use rust_kickstart::bank::BankService;

/// Example function demonstrating proper module usage
pub async fn demonstrate_architecture(user_service: UserService) {
    println!("=== Demonstrating Modular Architecture ===");
    
    // ✅ ALLOWED: Bank module can use UserService
    let bank_service = BankService::new(user_service.clone());
    
    // ✅ ALLOWED: Bank can create accounts through UserService
    match bank_service.create_account(1, 1000.0).await {
        Ok(message) => println!("✅ Bank: {}", message),
        Err(e) => println!("❌ Bank: {}", e),
    }
    
    // ✅ ALLOWED: Bank can get user information through UserService
    match bank_service.get_account_holder_name(1).await {
        Ok(name) => println!("✅ Bank: Account holder name: {}", name),
        Err(e) => println!("❌ Bank: {}", e),
    }
    
    // ✅ ALLOWED: Direct use of UserService
    match user_service.get_all_users().await {
        Ok(users) => println!("✅ Direct UserService: Found {} users", users.len()),
        Err(e) => println!("❌ Direct UserService: {}", e),
    }
    
    println!("\n=== What's NOT allowed (would cause compile errors) ===");
    println!("❌ Bank cannot access UserRepository directly");
    println!("❌ Bank cannot access user::repository::UserRepository");
    println!("❌ Other modules cannot import user::repository");
    println!("✅ Only UserService is exposed from the user module");
    
    println!("\n=== Architecture Benefits ===");
    println!("🔒 Encapsulation: Repository is hidden from other modules");
    println!("🎯 Single Responsibility: Each layer has a clear purpose");
    println!("🔄 Testability: Easy to mock UserService for testing");
    println!("📦 Modularity: Clean separation between business domains");
}

#[tokio::main]
async fn main() {
    println!("This is an example demonstrating the modular architecture.");
    println!("To see the actual demonstration, run the integration tests:");
    println!("cargo test --test integration_bank_user");
}