use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: Option<String>,
    pub aud: Option<String>,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    ExpiredToken,
    InvalidApiKey,
    MissingCredentials,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid JWT token"),
            AuthError::ExpiredToken => write!(f, "JWT token has expired"),
            AuthError::InvalidApiKey => write!(f, "Invalid API key"),
            AuthError::MissingCredentials => write!(f, "Missing authentication credentials"),
        }
    }
}

impl std::error::Error for AuthError {}

pub struct AuthService;

impl AuthService {
    pub fn validate_jwt_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);
        
        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => {
                match err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(AuthError::ExpiredToken),
                    _ => Err(AuthError::InvalidToken),
                }
            }
        }
    }

    pub async fn validate_api_key(api_key: &str) -> Result<ApiKeyInfo, AuthError> {
        // In a real implementation, this would query a database or cache
        // For demo purposes, we'll use a hardcoded set of valid API keys
        let valid_keys = get_valid_api_keys();
        
        if let Some(key_info) = valid_keys.get(api_key) {
            Ok(key_info.clone())
        } else {
            Err(AuthError::InvalidApiKey)
        }
    }

    pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }

    pub fn validate_permissions(required_permissions: &[&str], user_permissions: &[String]) -> bool {
        let user_perms: HashSet<&str> = user_permissions.iter().map(|s| s.as_str()).collect();
        
        required_permissions.iter().all(|perm| user_perms.contains(perm))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub key_id: String,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub rate_limit: u32,
    pub expires_at: Option<u64>,
    pub is_active: bool,
}

// In a real implementation, this would be loaded from a database
fn get_valid_api_keys() -> std::collections::HashMap<String, ApiKeyInfo> {
    let mut keys = std::collections::HashMap::new();
    
    keys.insert(
        "ak_admin_12345678901234567890".to_string(),
        ApiKeyInfo {
            key_id: "admin_key".to_string(),
            user_id: Some("admin".to_string()),
            permissions: vec![
                "admin".to_string(),
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
            ],
            rate_limit: 10000,
            expires_at: None,
            is_active: true,
        },
    );
    
    keys.insert(
        "ak_user_09876543210987654321".to_string(),
        ApiKeyInfo {
            key_id: "user_key".to_string(),
            user_id: Some("user".to_string()),
            permissions: vec![
                "read".to_string(),
            ],
            rate_limit: 1000,
            expires_at: None,
            is_active: true,
        },
    );
    
    keys.insert(
        "ak_service_11111111111111111111".to_string(),
        ApiKeyInfo {
            key_id: "service_key".to_string(),
            user_id: None,
            permissions: vec![
                "service".to_string(),
                "read".to_string(),
                "write".to_string(),
            ],
            rate_limit: 5000,
            expires_at: None,
            is_active: true,
        },
    );
    
    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    #[test]
    fn test_valid_jwt_token() {
        let secret = "test_secret";
        let claims = Claims {
            sub: "test_user".to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
            iss: None,
            aud: None,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ).unwrap();
        
        let result = AuthService::validate_jwt_token(&token, secret);
        assert!(result.is_ok());
        
        let decoded_claims = result.unwrap();
        assert_eq!(decoded_claims.sub, "test_user");
    }

    #[test]
    fn test_invalid_jwt_token() {
        let secret = "test_secret";
        let invalid_token = "invalid.token.here";
        
        let result = AuthService::validate_jwt_token(invalid_token, secret);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_valid_api_key() {
        let api_key = "ak_admin_12345678901234567890";
        let result = AuthService::validate_api_key(api_key).await;
        
        assert!(result.is_ok());
        let key_info = result.unwrap();
        assert_eq!(key_info.key_id, "admin_key");
        assert!(key_info.permissions.contains(&"admin".to_string()));
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        let api_key = "invalid_key";
        let result = AuthService::validate_api_key(api_key).await;
        
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bearer_token() {
        let auth_header = "Bearer abc123def456";
        let token = AuthService::extract_bearer_token(auth_header);
        
        assert_eq!(token, Some("abc123def456"));
    }

    #[test]
    fn test_validate_permissions() {
        let required = vec!["read", "write"];
        let user_permissions = vec!["read".to_string(), "write".to_string(), "admin".to_string()];
        
        assert!(AuthService::validate_permissions(&required, &user_permissions));
        
        let insufficient_permissions = vec!["read".to_string()];
        assert!(!AuthService::validate_permissions(&required, &insufficient_permissions));
    }
} 