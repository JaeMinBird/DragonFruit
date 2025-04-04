use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use time::OffsetDateTime;

// Simplified time serializer without external dependency
mod datetime_serializer {
    use serde::{Deserialize, Deserializer, Serializer, Serialize};
    use time::OffsetDateTime;
    
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        date.to_string().serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
            .map_err(serde::de::Error::custom)
    }
    
    pub mod option {
        use super::*;
        
        pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match date {
                Some(date) => date.to_string().serialize(serializer),
                None => serializer.serialize_none(),
            }
        }
        
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;
            match s {
                Some(s) => OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
                    .map(Some)
                    .map_err(serde::de::Error::custom),
                None => Ok(None),
            }
        }
    }
}

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