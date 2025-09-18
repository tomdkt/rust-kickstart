//! Business rule validators
//! 
//! Contains reusable validation rules that can be applied to different fields and contexts.


use super::common::{field_error, ValidationResult};

/// Validates a name field
pub fn validate_name(name: &str, field_name: &str) -> ValidationResult {
    let mut errors = Vec::new();
    
    if name.trim().is_empty() {
        errors.push(field_error(field_name, "Name cannot be empty"));
    }
    
    if name.len() > 100 {
        errors.push(field_error(field_name, "Name cannot exceed 100 characters"));
    }
    
    // Additional name validation rules can be added here
    if name.chars().any(|c| c.is_numeric()) {
        errors.push(field_error(field_name, "Name cannot contain numbers"));
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates an age field
pub fn validate_age(age: i32, field_name: &str) -> ValidationResult {
    let mut errors = Vec::new();
    
    if age < 0 {
        errors.push(field_error(field_name, "Age cannot be negative"));
    }
    
    if age > 150 {
        errors.push(field_error(field_name, "Age cannot exceed 150 years"));
    }
    
    // Additional age validation rules can be added here
    if age == 0 {
        errors.push(field_error(field_name, "Age must be greater than 0"));
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates that a string contains only allowed characters
pub fn validate_allowed_characters(value: &str, field_name: &str, allowed_chars: &str) -> ValidationResult {
    if value.chars().all(|c| allowed_chars.contains(c) || c.is_alphabetic() || c.is_whitespace()) {
        Ok(())
    } else {
        Err(vec![field_error(field_name, format!("Field contains invalid characters. Allowed: {}", allowed_chars))])
    }
}

/// Validates minimum length
pub fn validate_min_length(value: &str, field_name: &str, min_length: usize) -> ValidationResult {
    if value.len() >= min_length {
        Ok(())
    } else {
        Err(vec![field_error(field_name, format!("Field must be at least {} characters long", min_length))])
    }
}

/// Validates maximum length
pub fn validate_max_length(value: &str, field_name: &str, max_length: usize) -> ValidationResult {
    if value.len() <= max_length {
        Ok(())
    } else {
        Err(vec![field_error(field_name, format!("Field cannot exceed {} characters", max_length))])
    }
}

/// Validates a range for numeric values
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T, 
    field_name: &str, 
    min: T, 
    max: T
) -> ValidationResult {
    let mut errors = Vec::new();
    
    if value < min {
        errors.push(field_error(field_name, format!("Value must be at least {}", min)));
    }
    
    if value > max {
        errors.push(field_error(field_name, format!("Value cannot exceed {}", max)));
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}