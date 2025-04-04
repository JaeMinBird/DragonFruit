use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use time::OffsetDateTime;

use crate::utils::time::datetime_serializer;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Credential {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetime_serializer")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCredential {
    pub category_id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCredential {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialResponse {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub website: Option<String>,
    pub username: String,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetime_serializer")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialWithPassword {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub website: Option<String>,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetime_serializer")]
    pub updated_at: OffsetDateTime,
}

impl From<Credential> for CredentialResponse {
    fn from(credential: Credential) -> Self {
        Self {
            id: credential.id,
            category_id: Some(credential.category_id),
            name: credential.name,
            website: credential.website,
            username: credential.username,
            created_at: credential.created_at,
            updated_at: credential.updated_at,
        }
    }
} 