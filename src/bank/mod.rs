//! Bank module
//! 
//! This module demonstrates how other modules can use UserService
//! but cannot access UserRepository directly.

pub mod service;

// Public exports
pub use service::{BankService, BankError};