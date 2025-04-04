use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::errors::AppError;

// Configuration for JWT tokens
#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub algorithm: Algorithm,
    pub expires_in: i64, // in seconds
    pub issuer: String,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::HS256,
            expires_in: 60 * 60 * 24, // 24 hours
            issuer: "dragonfruit".to_string(),
        }
    }
}

// Claims for JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     // Subject (user ID)
    pub iss: String,     // Issuer
    pub iat: i64,        // Issued at (timestamp)
    pub exp: i64,        // Expiration time (timestamp)
}

// Create a JWT token for a user
pub fn create_token(user_id: Uuid, config: &TokenConfig) -> Result<String, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let expires_at = now + config.expires_in;
    
    let claims = Claims {
        sub: user_id.to_string(),
        iss: config.issuer.clone(),
        iat: now,
        exp: expires_at,
    };
    
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    
    encode(
        &Header::new(config.algorithm),
        &claims,
        &key,
    )
    .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))
}

// Validate a JWT token
pub fn validate_token(token: &str) -> Result<Uuid, AppError> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(token, &key, &validation)
        .map_err(|e| {
            if e.to_string().contains("expired") {
                AppError::Unauthorized("Token expired".to_string())
            } else {
                AppError::Unauthorized("Invalid token".to_string())
            }
        })?;
    
    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;
    
    Ok(user_id)
} 