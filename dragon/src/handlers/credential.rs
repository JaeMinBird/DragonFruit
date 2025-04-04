use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    crypto::{encrypt_password, decrypt_password},
    errors::AppError,
    middleware::auth::AuthUser,
    models::credential::{
        Credential, CreateCredential, CredentialResponse, 
        CredentialWithPassword, UpdateCredential
    },
};

// Get all credentials for a user
pub async fn get_credentials(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let credentials = sqlx::query_as::<_, Credential>(
        "SELECT * FROM credentials WHERE user_id = $1 ORDER BY name",
    )
    .bind(auth_user.user_id)
    .fetch_all(&pool)
    .await?;

    // Convert to response format (without passwords)
    let credentials_response: Vec<CredentialResponse> = credentials
        .into_iter()
        .map(|c| c.into())
        .collect();

    Ok((StatusCode::OK, Json(credentials_response)))
}

// Get a single credential by ID
pub async fn get_credential(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(credential_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let credential = sqlx::query_as::<_, Credential>(
        "SELECT * FROM credentials WHERE id = $1 AND user_id = $2",
    )
    .bind(credential_id)
    .bind(auth_user.user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    let response = CredentialResponse::from(credential);
    Ok((StatusCode::OK, Json(response)))
}

// Get a single credential with its decrypted password
pub async fn get_credential_with_password(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(credential_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let credential = sqlx::query_as::<_, Credential>(
        "SELECT * FROM credentials WHERE id = $1 AND user_id = $2",
    )
    .bind(credential_id)
    .bind(auth_user.user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    // Decrypt the password
    let password = decrypt_password(&credential.password, &auth_user.user_id.to_string())
        .map_err(|e| AppError::Internal(format!("Failed to decrypt password: {}", e)))?;

    // Build the full response including the decrypted password
    let response = CredentialWithPassword {
        id: credential.id,
        category_id: Some(credential.category_id),
        name: credential.name,
        website: credential.website,
        username: credential.username,
        password,
        notes: credential.notes,
        created_at: credential.created_at,
        updated_at: credential.updated_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

// Create a new credential
pub async fn create_credential(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCredential>,
) -> Result<impl IntoResponse, AppError> {
    // Validate inputs
    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("Credential name cannot be empty".to_string()));
    }

    // Encrypt the password
    let encrypted_password = encrypt_password(&payload.password, &auth_user.user_id.to_string())
        .map_err(|e| AppError::Internal(format!("Failed to encrypt password: {}", e)))?;

    // Create the credential
    let credential = sqlx::query_as::<_, Credential>(
        r#"
        INSERT INTO credentials (
            user_id, category_id, name, username, 
            password, website, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(auth_user.user_id)
    .bind(payload.category_id)
    .bind(&payload.name)
    .bind(&payload.username)
    .bind(&encrypted_password)
    .bind(&payload.website)
    .bind(&payload.notes)
    .fetch_one(&pool)
    .await?;

    // Convert to response format
    let response = CredentialResponse::from(credential);
    Ok((StatusCode::CREATED, Json(response)))
}

// Update a credential
pub async fn update_credential(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(credential_id): Path<Uuid>,
    Json(payload): Json<UpdateCredential>,
) -> Result<impl IntoResponse, AppError> {
    // Check if credential exists and belongs to user
    let _credential = sqlx::query_as::<_, Credential>(
        "SELECT * FROM credentials WHERE id = $1 AND user_id = $2",
    )
    .bind(credential_id)
    .bind(auth_user.user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    // Name validation
    if let Some(ref name) = payload.name {
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("Credential name cannot be empty".to_string()));
        }
    }

    // Handle password encryption if provided
    let encrypted_password = if let Some(ref password) = payload.password {
        Some(encrypt_password(password, &auth_user.user_id.to_string())
            .map_err(|e| AppError::Internal(format!("Failed to encrypt password: {}", e)))?)
    } else {
        None
    };

    // Build update query dynamically
    let mut query = "UPDATE credentials SET ".to_string();
    let mut params = Vec::new();
    let mut param_count = 1;

    // Add name if provided
    if let Some(name) = &payload.name {
        query += &format!("name = ${}", param_count);
        params.push(name.clone());
        param_count += 1;
    }

    // Add username if provided
    if let Some(username) = &payload.username {
        if param_count > 1 {
            query += ", ";
        }
        query += &format!("username = ${}", param_count);
        params.push(username.clone());
        param_count += 1;
    }

    // Add password if provided
    if let Some(password) = encrypted_password {
        if param_count > 1 {
            query += ", ";
        }
        query += &format!("password = ${}", param_count);
        params.push(password);
        param_count += 1;
    }

    // Add website if provided
    if let Some(website) = &payload.website {
        if param_count > 1 {
            query += ", ";
        }
        query += &format!("website = ${}", param_count);
        params.push(website.clone());
        param_count += 1;
    }

    // Add notes if provided
    if let Some(notes) = &payload.notes {
        if param_count > 1 {
            query += ", ";
        }
        query += &format!("notes = ${}", param_count);
        params.push(notes.clone());
        param_count += 1;
    }

    // Add category_id if provided
    if let Some(category_id) = payload.category_id {
        if param_count > 1 {
            query += ", ";
        }
        query += &format!("category_id = ${}", param_count);
        params.push(category_id.to_string());
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
        
        // Add credential ID and user ID
        query_builder = query_builder.bind(credential_id).bind(auth_user.user_id);
        
        // Execute
        query_builder.execute(&pool).await?;
    }

    // Get the updated credential
    let updated_credential = sqlx::query_as::<_, Credential>(
        "SELECT * FROM credentials WHERE id = $1 AND user_id = $2",
    )
    .bind(credential_id)
    .bind(auth_user.user_id)
    .fetch_one(&pool)
    .await?;

    // Return updated credential
    let response = CredentialResponse::from(updated_credential);
    Ok((StatusCode::OK, Json(response)))
}

// Delete a credential
pub async fn delete_credential(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
    Path(credential_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Delete the credential
    let result = sqlx::query(
        "DELETE FROM credentials WHERE id = $1 AND user_id = $2",
    )
    .bind(credential_id)
    .bind(auth_user.user_id)
    .execute(&pool)
    .await?;

    // Check if anything was deleted
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Credential not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
} 