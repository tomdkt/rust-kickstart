# Modular Architecture Guide

This project demonstrates a clean modular architecture in Rust, similar to NestJS modules, with proper separation of concerns and encapsulation.

## Architecture Overview

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Main library with router setup
├── user/                # User module (complete feature)
│   ├── mod.rs           # Module exports (only UserService is public)
│   ├── domain.rs        # Domain models, validation, errors
│   ├── repository.rs    # Database operations (private to module)
│   ├── service.rs       # Business logic (public interface)
│   └── controller.rs    # HTTP handlers (used internally)
├── bank/                # Bank module (demonstrates inter-module usage)
│   ├── mod.rs           # Module exports
│   └── service.rs       # Bank business logic using UserService
└── example.rs           # Architecture demonstration
```

## Key Principles

### 1. Encapsulation
- **UserRepository** is `pub(super)` - only accessible within the user module
- **UserService** is `pub` - the main interface for other modules
- **Controllers** are used internally but exposed for OpenAPI documentation

### 2. Dependency Direction
```rust
// ✅ ALLOWED: Other modules can use UserService
use crate::user::UserService;

// ❌ NOT ALLOWED: Direct repository access (would cause compile error)
use crate::user::repository::UserRepository; // This won't compile!
```

### 3. Layer Responsibilities

#### Domain Layer (`domain.rs`)
- Data models (`User`, `CreateUser`, `UpdateUser`)
- Validation logic
- Domain errors
- Business rules

#### Repository Layer (`repository.rs`)
- Database operations
- Data persistence
- SQL queries
- Private to the module (`pub(super)`)

#### Service Layer (`service.rs`)
- Business logic coordination
- Validation orchestration
- Error handling
- Public interface for other modules

#### Controller Layer (`controller.rs`)
- HTTP request/response handling
- Status code mapping
- OpenAPI documentation
- Used internally by the router

## Usage Examples

### Creating Services
```rust
// In lib.rs - setting up the application
let user_service = UserService::new(pool.clone());
let bank_service = BankService::new(user_service.clone());
```

### Inter-Module Communication
```rust
// Bank module using UserService (ALLOWED)
impl BankService {
    pub async fn create_account(&self, user_id: i32, balance: f64) -> Result<String, BankError> {
        // ✅ Can check if user exists through UserService
        if self.user_service.user_exists(user_id).await? {
            Ok(format!("Account created for user {}", user_id))
        } else {
            Err(BankError::UserNotFound)
        }
    }
}
```

### What's NOT Allowed
```rust
// ❌ This would cause a compile error:
// use crate::user::repository::UserRepository;

// ❌ Bank cannot directly access database
// let user = sqlx::query!("SELECT * FROM users WHERE id = ?", user_id);

// ✅ Must go through UserService instead
// let user = self.user_service.get_user_by_id(user_id).await?;
```

## Benefits

### 🔒 **Encapsulation**
- Repository layer is completely hidden from other modules
- Database access is centralized and controlled
- Implementation details can change without affecting other modules

### 🎯 **Single Responsibility**
- Each layer has a clear, focused purpose
- Domain logic is separated from persistence logic
- HTTP concerns are isolated in controllers

### 🔄 **Testability**
- Easy to mock UserService for testing other modules
- Repository can be tested independently
- Clear boundaries make unit testing straightforward

### 📦 **Modularity**
- Features are self-contained modules
- Easy to add new modules that depend on existing services
- Clear interfaces between modules

### 🛡️ **Type Safety**
- Rust's module system enforces architectural boundaries at compile time
- Impossible to accidentally bypass the service layer
- Refactoring is safer with compiler guarantees

## Adding New Modules

To add a new module (e.g., `order`):

1. Create the module structure:
```rust
src/order/
├── mod.rs           # Export OrderService only
├── domain.rs        # Order models and validation
├── repository.rs    # Order database operations (private)
├── service.rs       # Order business logic (public)
└── controller.rs    # Order HTTP handlers
```

2. Import required services:
```rust
// In order/service.rs
use crate::user::UserService;

impl OrderService {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }
    
    pub async fn create_order(&self, user_id: i32) -> Result<Order, OrderError> {
        // ✅ Can validate user exists through UserService
        self.user_service.user_exists(user_id).await?;
        // ... order creation logic
    }
}
```

3. Wire up in main application:
```rust
// In lib.rs
let user_service = UserService::new(pool.clone());
let order_service = OrderService::new(user_service.clone());
```

This architecture ensures clean separation of concerns while maintaining flexibility and type safety.