use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use time::OffsetDateTime;

use crate::utils::time::datetime_serializer;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetime_serializer")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCreate {
    pub name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUpdate {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    #[serde(with = "datetime_serializer")]
    pub created_at: OffsetDateTime,
    pub children: Option<Vec<CategoryResponse>>,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
            parent_id: category.parent_id,
            created_at: category.created_at,
            children: None,
        }
    }
} 