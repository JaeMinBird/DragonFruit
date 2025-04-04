use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use time::OffsetDateTime;

use crate::utils::time::datetime_serializer;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetime_serializer")]
    pub updated_at: OffsetDateTime,
    pub last_login: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub totp_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub totp_enabled: bool,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetime_serializer")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "datetime_serializer::option", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<OffsetDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            totp_enabled: user.totp_enabled,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_login: user.last_login,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
} 