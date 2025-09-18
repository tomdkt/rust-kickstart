# Integration Test: Bank + User Modules

## Overview

This integration test demonstrates the modular architecture in action by:

1. **Creating a user** through `UserService`
2. **Creating a bank account** for that user using `BankService` with balance $10.00
3. **Retrieving account information** that combines user and bank data
4. **Testing error scenarios** (non-existent users)
5. **Testing updates** through the bank service

## Test Structure

```rust
tests/
├── integration_user.rs           # User module HTTP endpoint tests
│   ├── test_get_all_users_empty()
│   ├── test_create_user()
│   ├── test_get_user_not_found()
│   └── test_full_crud_workflow()
└── integration_bank_user.rs      # Bank + User module interaction tests
    ├── test_bank_user_integration()
    ├── test_bank_service_with_nonexistent_user()
    └── test_bank_service_update_account_holder()
```

## What the Test Demonstrates

### ✅ **Modular Architecture**
- `BankService` uses `UserService` but cannot access `UserRepository` directly
- Clean separation of concerns between modules
- Type-safe inter-module communication

### ✅ **Business Logic Flow**
```rust
// 1. Create user
let user = user_service.create_user(create_data).await?;

// 2. Create bank account (uses UserService internally)
let account = bank_service.create_account(user.id, 10.00).await?;

// 3. Get account info (combines user + bank data)
let info = bank_service.get_account_info(user.id).await?;
```

### ✅ **Error Handling**
- Tests that non-existent users are properly rejected
- Demonstrates proper error propagation between modules

### ✅ **Data Consistency**
- Verifies that updates through `BankService` are reflected in `UserService`
- Tests that both services see the same data

## Running the Tests

```bash
# Run user HTTP endpoint tests
cargo test --test integration_user

# Run bank + user module interaction tests
cargo test --test integration_bank_user

# Run all integration tests
cargo test --test integration_

# Run all tests
make test
```

## Expected Output

```
running 3 tests
test test_bank_service_with_nonexistent_user ... ok
test test_bank_service_update_account_holder ... ok
test test_bank_user_integration ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Test Database Isolation

Each test creates its own isolated PostgreSQL schema:
- Schema name: `test_{uuid_v7}`
- Runs migrations automatically
- Cleans up after test completion
- Allows parallel test execution

## Architecture Benefits Demonstrated

### 🔒 **Encapsulation**
- Bank module cannot access user database directly
- Must go through `UserService` interface

### 🎯 **Single Responsibility**
- `UserService`: User management
- `BankService`: Bank operations + user validation

### 🔄 **Testability**
- Easy to test inter-module communication
- Clear boundaries for mocking

### 📦 **Modularity**
- Adding new modules follows the same pattern
- Services can be composed together

This test proves that the modular architecture works correctly and maintains proper separation of concerns while allowing clean inter-module communication.