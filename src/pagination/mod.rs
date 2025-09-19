//! Pagination token system
//!
//! Provides opaque pagination tokens for cursor-based pagination,

use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Internal cursor data structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CursorData {
    /// Last record ID
    pub id: i32,
    /// Timestamp for consistent ordering
    pub timestamp: DateTime<Utc>,
}

/// Pagination token for cursor-based pagination
#[derive(Debug, Clone)]
pub struct PaginationToken;

/// Errors that can occur during token operations
#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    /// Token is malformed or invalid
    #[error("Invalid pagination token")]
    InvalidToken,
    /// Token encoding/decoding failed
    #[error("Token encoding error: {0}")]
    EncodingError(String),
}

impl PaginationToken {
    /// Encodes pagination information into an opaque token
    pub fn encode(last_id: i32, timestamp: DateTime<Utc>) -> Result<String, TokenError> {
        let cursor_data = CursorData {
            id: last_id,
            timestamp,
        };

        let json = serde_json::to_string(&cursor_data)
            .map_err(|e| TokenError::EncodingError(e.to_string()))?;

        let token = general_purpose::URL_SAFE_NO_PAD.encode(json.as_bytes());
        Ok(token)
    }

    /// Decodes a pagination token to extract cursor information
    pub fn decode(token: &str) -> Result<(i32, DateTime<Utc>), TokenError> {
        let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| TokenError::InvalidToken)?;

        let json = String::from_utf8(decoded_bytes).map_err(|_| TokenError::InvalidToken)?;

        let cursor_data: CursorData =
            serde_json::from_str(&json).map_err(|_| TokenError::InvalidToken)?;

        Ok((cursor_data.id, cursor_data.timestamp))
    }

    /// Validates if a token is well-formed (without fully decoding)
    pub fn is_valid(token: &str) -> bool {
        Self::decode(token).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_token_encode_decode() {
        let id = 123;
        let timestamp = Utc::now();

        let token = PaginationToken::encode(id, timestamp).unwrap();
        let (decoded_id, decoded_timestamp) = PaginationToken::decode(&token).unwrap();

        assert_eq!(id, decoded_id);
        assert_eq!(timestamp, decoded_timestamp);
    }

    #[test]
    fn test_invalid_token() {
        assert!(PaginationToken::decode("invalid_token").is_err());
        assert!(!PaginationToken::is_valid("invalid_token"));
    }

    #[test]
    fn test_token_validation() {
        let token = PaginationToken::encode(456, Utc::now()).unwrap();
        assert!(PaginationToken::is_valid(&token));
    }
}
