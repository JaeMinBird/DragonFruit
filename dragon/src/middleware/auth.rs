use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    auth::jwt::validate_token,
    errors::AppError,
};

// Extract user ID from JWT token
pub struct AuthUser {
    pub user_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

        // Check if it's a Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid Authorization scheme".to_string()));
        }

        // Extract the token
        let token = &auth_header[7..];

        // Validate the token and extract the user ID
        let user_id = validate_token(token)?;

        Ok(AuthUser { user_id })
    }
}

// Middleware to require authentication
pub async fn require_auth<B>(
    auth_user: Result<AuthUser, AppError>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    // If the auth_user was extracted successfully, proceed
    match auth_user {
        Ok(_) => Ok(next.run(req).await),
        Err(e) => Err(e),
    }
} 