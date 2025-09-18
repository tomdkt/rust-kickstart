//! UpdateUser validation logic
//! 
//! Contains validation rules specific to user update operations.

use crate::user::domain::UpdateUser;
use super::common::{ValidationResult, ValidationContext, general_error};
use super::rules::{validate_name, validate_age};

/// Validates user update data
pub fn validate_update_user(user: &UpdateUser) -> ValidationResult {
    validate_update_user_with_context(user, &ValidationContext::new())
}

/// Validates user update data with additional context
pub fn validate_update_user_with_context(
    user: &UpdateUser, 
    _context: &ValidationContext
) -> ValidationResult {
    let mut all_errors = Vec::new();
    
    // Check if at least one field is provided for update
    if user.name.is_none() && user.age.is_none() {
        all_errors.push(general_error("At least one field (name or age) must be provided for update"));
        return Err(all_errors);
    }
    
    // Validate name if provided
    if let Some(ref name) = user.name {
        if let Err(mut errors) = validate_name(name, "name") {
            all_errors.append(&mut errors);
        }
    }
    
    // Validate age if provided
    if let Some(age) = user.age {
        if let Err(mut errors) = validate_age(age, "age") {
        all_errors.append(&mut errors);
        }
    }
    
    // Additional update-specific validations can be added here
    // For example, checking if the update would create duplicates, etc.
    
    if all_errors.is_empty() {
        Ok(())
    } else {
        Err(all_errors)
    }
}

/// Validates partial update data (allows empty updates for specific use cases)
pub fn validate_partial_update_user(user: &UpdateUser) -> ValidationResult {
    let mut all_errors = Vec::new();
    
    // For partial updates, we don't require at least one field
    // This is useful for conditional updates or when combined with other operations
    
    // Validate name if provided
    if let Some(ref name) = user.name {
        if let Err(mut errors) = validate_name(name, "name") {
            all_errors.append(&mut errors);
        }
    }
    
    // Validate age if provided
    if let Some(age) = user.age {
        if let Err(mut errors) = validate_age(age, "age") {
            all_errors.append(&mut errors);
        }
    }
    
    if all_errors.is_empty() {
        Ok(())
    } else {
        Err(all_errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_update_user() {
        let user = UpdateUser {
            name: Some("Jane Doe".to_string()),
            age: Some(30),
        };
        
        assert!(validate_update_user(&user).is_ok());
    }
    
    #[test]
    fn test_valid_update_user_name_only() {
        let user = UpdateUser {
            name: Some("Jane Doe".to_string()),
            age: None,
        };
        
        assert!(validate_update_user(&user).is_ok());
    }
    
    #[test]
    fn test_invalid_update_user_no_fields() {
        let user = UpdateUser {
            name: None,
            age: None,
        };
        
        let result = validate_update_user(&user);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].field.is_none()); // General error
    }
    
    #[test]
    fn test_invalid_update_user_empty_name() {
        let user = UpdateUser {
            name: Some("".to_string()),
            age: Some(25),
        };
        
        let result = validate_update_user(&user);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == Some("name".to_string())));
    }
    
    #[test]
    fn test_partial_update_allows_empty() {
        let user = UpdateUser {
            name: None,
            age: None,
        };
        
        assert!(validate_partial_update_user(&user).is_ok());
    }
}