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
    if name.chars().any(char::is_numeric) {
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
        Err(vec![field_error(field_name, format!("Field contains invalid characters. Allowed: {allowed_chars}"))])
    }
}

/// Validates minimum length
pub fn validate_min_length(value: &str, field_name: &str, min_length: usize) -> ValidationResult {
    if value.len() >= min_length {
        Ok(())
    } else {
        Err(vec![field_error(field_name, format!("Field must be at least {min_length} characters long"))])
    }
}

/// Validates maximum length
pub fn validate_max_length(value: &str, field_name: &str, max_length: usize) -> ValidationResult {
    if value.len() <= max_length {
        Ok(())
    } else {
        Err(vec![field_error(field_name, format!("Field cannot exceed {max_length} characters"))])
    }
}

/// Validates a range for numeric values
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: &T, 
    field_name: &str, 
    min: &T, 
    max: &T
) -> ValidationResult {
    let mut errors = Vec::new();
    
    if value < min {
        errors.push(field_error(field_name, format!("Value must be at least {min}")));
    }
    
    if value > max {
        errors.push(field_error(field_name, format!("Value cannot exceed {max}")));
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_valid() {
        assert!(validate_name("John Doe", "name").is_ok());
        assert!(validate_name("Mary Jane", "name").is_ok());
        assert!(validate_name("José", "name").is_ok());
    }

    #[test]
    fn test_validate_name_empty() {
        let result = validate_name("", "name");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, Some("name".to_string()));
        assert!(errors[0].message.contains("cannot be empty"));
    }

    #[test]
    fn test_validate_name_whitespace_only() {
        let result = validate_name("   ", "name");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("cannot be empty"));
    }

    #[test]
    fn test_validate_name_too_long() {
        let long_name = "a".repeat(101);
        let result = validate_name(&long_name, "name");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("cannot exceed 100 characters")));
    }

    #[test]
    fn test_validate_name_with_numbers() {
        let result = validate_name("John123", "name");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("cannot contain numbers")));
    }

    #[test]
    fn test_validate_name_multiple_errors() {
        let result = validate_name("123", "name");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1); // Only the numbers error since it's not empty
        assert!(errors[0].message.contains("cannot contain numbers"));
    }

    #[test]
    fn test_validate_age_valid() {
        assert!(validate_age(25, "age").is_ok());
        assert!(validate_age(1, "age").is_ok());
        assert!(validate_age(150, "age").is_ok());
    }

    #[test]
    fn test_validate_age_negative() {
        let result = validate_age(-1, "age");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("cannot be negative")));
    }

    #[test]
    fn test_validate_age_zero() {
        let result = validate_age(0, "age");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("must be greater than 0")));
    }

    #[test]
    fn test_validate_age_too_high() {
        let result = validate_age(151, "age");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("cannot exceed 150 years")));
    }

    #[test]
    fn test_validate_allowed_characters_valid() {
        assert!(validate_allowed_characters("Hello World", "field", "!@#").is_ok());
        assert!(validate_allowed_characters("Test", "field", "").is_ok());
    }

    #[test]
    fn test_validate_allowed_characters_invalid() {
        let result = validate_allowed_characters("Hello@World", "field", "");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].message.contains("invalid characters"));
    }

    #[test]
    fn test_validate_min_length_valid() {
        assert!(validate_min_length("hello", "field", 3).is_ok());
        assert!(validate_min_length("hello", "field", 5).is_ok());
    }

    #[test]
    fn test_validate_min_length_invalid() {
        let result = validate_min_length("hi", "field", 5);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].message.contains("at least 5 characters"));
    }

    #[test]
    fn test_validate_max_length_valid() {
        assert!(validate_max_length("hello", "field", 10).is_ok());
        assert!(validate_max_length("hello", "field", 5).is_ok());
    }

    #[test]
    fn test_validate_max_length_invalid() {
        let result = validate_max_length("hello world", "field", 5);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].message.contains("cannot exceed 5 characters"));
    }

    #[test]
    fn test_validate_range_valid() {
        assert!(validate_range(&5, "field", &1, &10).is_ok());
        assert!(validate_range(&1, "field", &1, &10).is_ok());
        assert!(validate_range(&10, "field", &1, &10).is_ok());
    }

    #[test]
    fn test_validate_range_below_min() {
        let result = validate_range(&0, "field", &1, &10);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("at least 1")));
    }

    #[test]
    fn test_validate_range_above_max() {
        let result = validate_range(&11, "field", &1, &10);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("cannot exceed 10")));
    }

    #[test]
    fn test_validate_range_both_errors() {
        let result = validate_range(&-5, "field", &1, &10);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1); // Only min error since -5 < 1
        assert!(errors[0].message.contains("at least 1"));
    }
}