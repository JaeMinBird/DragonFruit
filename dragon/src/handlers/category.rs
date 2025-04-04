use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::category::{Category, CategoryResponse, CreateCategory, UpdateCategory},
};

// Get all categories for a user
pub async fn get_categories(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE user_id = $1 ORDER BY name",
    )
    .bind(auth_user.user_id)
    .fetch_all(&pool)
    .await?;

    // Convert to response format
    let categories_response: Vec<CategoryResponse> = categories
        .into_iter()
        .map(|c| CategoryResponse {
            id: c.id,
            name: c.name,
            parent_id: None, // No parent_id in current schema
            created_at: c.created_at,
            children: None,
        })
        .collect();

    // Build the response
    Ok((StatusCode::OK, Json(categories_response)))
}

// Get a single category by ID
pub async fn get_category(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(category_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Get the category
    let category = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE id = $1 AND user_id = $2",
    )
    .bind(category_id)
    .bind(auth_user.user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    // Convert to response format
    let category_response = CategoryResponse {
        id: category.id,
        name: category.name,
        parent_id: None, // No parent_id in current schema
        created_at: category.created_at,
        children: None,
    };

    Ok((StatusCode::OK, Json(category_response)))
}

// Create a new category
pub async fn create_category(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCategory>,
) -> Result<impl IntoResponse, AppError> {
    // Validate inputs
    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("Category name cannot be empty".to_string()));
    }

    // Create the category
    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (user_id, name, description)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await?;

    // Convert to response format
    let category_response = CategoryResponse {
        id: category.id,
        name: category.name,
        parent_id: None, // No parent_id in current schema
        created_at: category.created_at,
        children: None,
    };

    Ok((StatusCode::CREATED, Json(category_response)))
}

// Update a category
pub async fn update_category(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(category_id): Path<Uuid>,
    Json(payload): Json<UpdateCategory>,
) -> Result<impl IntoResponse, AppError> {
    // Check if category exists and belongs to user
    let category_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1 AND user_id = $2)",
    )
    .bind(category_id)
    .bind(auth_user.user_id)
    .fetch_one(&pool)
    .await?;

    if !category_exists {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    // If name is provided, validate it
    if let Some(ref name) = payload.name {
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("Category name cannot be empty".to_string()));
        }
    }

    // Build update query dynamically based on provided fields
    let mut query = "UPDATE categories SET ".to_string();
    let mut params = Vec::new();
    let mut param_count = 1;

    // Add name if provided
    if let Some(name) = &payload.name {
        query += &format!("name = ${}", param_count);
        params.push(name.clone());
        param_count += 1;
    }

    // Add description if provided
    if let Some(description) = &payload.description {
        if param_count > 1 {
            query += ", ";
        }
        query += &format!("description = ${}", param_count);
        params.push(description.clone());
        param_count += 1;
    }

    // Add updated_at
    if param_count > 1 {
        query += ", ";
    }
    query += &format!("updated_at = now() WHERE id = ${} AND user_id = ${}", 
        param_count, param_count + 1);

    // Execute the query if any fields were provided
    if param_count > 1 {
        let mut query_builder = sqlx::query(&query);
        
        // Add all the parameters
        for param in params {
            query_builder = query_builder.bind(param);
        }
        
        // Add category ID and user ID
        query_builder = query_builder.bind(category_id).bind(auth_user.user_id);
        
        // Execute
        query_builder.execute(&pool).await?;
    }

    // Get the updated category
    let category = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE id = $1 AND user_id = $2",
    )
    .bind(category_id)
    .bind(auth_user.user_id)
    .fetch_one(&pool)
    .await?;

    // Convert to response format
    let category_response = CategoryResponse {
        id: category.id,
        name: category.name,
        parent_id: None, // No parent_id in current schema
        created_at: category.created_at,
        children: None,
    };

    Ok((StatusCode::OK, Json(category_response)))
}

// Delete a category
pub async fn delete_category(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(category_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Delete the category (but only if it belongs to the authenticated user)
    let result = sqlx::query(
        "DELETE FROM categories WHERE id = $1 AND user_id = $2",
    )
    .bind(category_id)
    .bind(auth_user.user_id)
    .execute(&pool)
    .await?;

    // Check if anything was deleted
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
} 