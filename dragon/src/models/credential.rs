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