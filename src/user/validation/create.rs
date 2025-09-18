//! `CreateUser` validation logic
//! 
//! Contains validation rules specific to user creation operations.

use crate::user::domain::{CreateUser, ValidationError};
use super::common::{ValidationResult, ValidationContext};
use super::rules::{validate_name, validate_age};

/// Validates user creation data
pub fn validate_create_user(user: &CreateUser) -> ValidationResult {
    validate_create_user_with_context(user, &ValidationContext::new())
}

/// Validates user creation data with additional context
pub fn validate_create_user_with_context(
    user: &CreateUser, 
    _context: &ValidationContext
) -> ValidationResult {
    let mut all_errors = Vec::new();
    
    // Validate name
    if let Err(mut errors) = validate_name(&user.name, "name") {
        all_errors.append(&mut errors);
    }
    
    // Validate age
    if let Err(mut errors) = validate_age(user.age, "age") {
        all_errors.append(&mut errors);
    }
    
    // Additional create-specific validations can be added here
    // For example, checking for duplicate names, business rules, etc.
    
    if all_errors.is_empty() {
        Ok(())
    } else {
        Err(all_errors)
    }
}

/// Validates user creation data for batch operations
pub fn validate_create_user_batch(users: &[CreateUser]) -> Result<(), Vec<(usize, Vec<ValidationError>)>> {
    let mut batch_errors = Vec::new();
    
    for (index, user) in users.iter().enumerate() {
        if let Err(errors) = validate_create_user(user) {
            batch_errors.push((index, errors));
        }
    }
    
    if batch_errors.is_empty() {
        Ok(())
    } else {
        Err(batch_errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_create_user() {
        let user = CreateUser {
            name: "John Doe".to_string(),
            age: 25,
        };
        
        assert!(validate_create_user(&user).is_ok());
    }
    
    #[test]
    fn test_invalid_create_user_empty_name() {
        let user = CreateUser {
            name: "".to_string(),
            age: 25,
        };
        
        let result = validate_create_user(&user);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, Some("name".to_string()));
    }
    
    #[test]
    fn test_invalid_create_user_negative_age() {
        let user = CreateUser {
            name: "John Doe".to_string(),
            age: -5,
        };
        
        let result = validate_create_user(&user);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == Some("age".to_string())));
    }
}