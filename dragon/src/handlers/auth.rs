use axum::{
    extract::{Json, State},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::{
    auth::{
        jwt::{create_token, TokenConfig},
        password::{hash_password, verify_password},
        totp::{generate_totp_secret, verify_totp},
    },
    errors::AppError,
    models::user::{
        User, CreateUser, LoginUser, UserResponse,
    },
    middleware::auth::AuthUser,
};

// Register a new user
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let password_hash = hash_password(&payload.password)?;
    
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, totp_secret, totp_enabled, created_at, updated_at, last_login
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            if e.to_string().contains("users_email_key") {
                AppError::Conflict("Email already exists".to_string())
            } else if e.to_string().contains("users_username_key") {
                AppError::Conflict("Username already exists".to_string())
            } else {
                AppError::Database(e)
            }
        } else {
            AppError::Database(e)
        }
    })?;

    // Respond with user info
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

// Login a user
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    // Find user by username
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Verify password
    let password_verified = verify_password(&payload.password, &user.password_hash)?;
    
    if !password_verified {
        return Err(AppError::Unauthorized("Invalid password".to_string()));
    }

    // If TOTP is enabled, verify the TOTP code
    if user.totp_enabled {
        let totp_secret = user.totp_secret.as_ref()
            .ok_or_else(|| AppError::Internal("TOTP secret not found".to_string()))?;
        
        let totp_code = payload.totp_code
            .ok_or_else(|| AppError::BadRequest("TOTP code required".to_string()))?;
        
        if !verify_totp(totp_secret, &totp_code, 30, 6)? {
            return Err(AppError::Unauthorized("Invalid TOTP code".to_string()));
        }
    }

    // Update last login time
    let now = OffsetDateTime::now_utc();
    sqlx::query("UPDATE users SET last_login = $1 WHERE id = $2")
        .bind(now)
        .bind(user.id)
        .execute(&pool)
        .await?;

    // Create JWT token
    let token_config = TokenConfig::default();
    let token = create_token(user.id, &token_config)?;

    // Return user and token
    Ok((
        StatusCode::OK, 
        Json(serde_json::json!({
            "user": UserResponse::from(user),
            "token": token
        }))
    ))
}

// Generate TOTP secret for a user
pub async fn generate_totp_for_user(
    State(pool): State<PgPool>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    // Generate TOTP secret
    let (secret, uri) = generate_totp_secret("DragonFruit", &auth_user.user_id.to_string())?;
    
    // Update user with TOTP secret (but don't enable it yet)
    sqlx::query("UPDATE users SET totp_secret = $1 WHERE id = $2")
        .bind(&secret)
        .bind(auth_user.user_id)
        .execute(&pool)
        .await?;
    
    // Return the TOTP secret and URI
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "secret": secret,
            "uri": uri
        }))
    ))
}

// Enable TOTP for a user after verification
pub async fn enable_totp(
    State(pool): State<PgPool>,
    auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // Get the user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&pool)
        .await?;
    
    // Get TOTP code from payload
    let totp_code = payload["code"].as_str()
        .ok_or_else(|| AppError::BadRequest("TOTP code required".to_string()))?;
    
    // Verify the TOTP code
    let totp_secret = user.totp_secret.as_ref()
        .ok_or_else(|| AppError::BadRequest("TOTP not set up yet".to_string()))?;
    
    if !verify_totp(totp_secret, totp_code, 30, 6)? {
        return Err(AppError::BadRequest("Invalid TOTP code".to_string()));
    }
    
    // Enable TOTP
    sqlx::query("UPDATE users SET totp_enabled = true WHERE id = $1")
        .bind(auth_user.user_id)
        .execute(&pool)
        .await?;
    
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "TOTP enabled successfully"
        }))
    ))
}

// Get user profile
pub async fn get_profile(
    State(pool): State<PgPool>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&pool)
        .await?;
    
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

// Update user profile
pub async fn update_profile(
    State(pool): State<PgPool>,
    auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // Build the update query dynamically based on provided fields
    let mut query = "UPDATE users SET ".to_string();
    let mut binds = vec![];
    let mut i = 1;
    
    // Username update
    if let Some(username) = payload["username"].as_str() {
        if i > 1 { query.push_str(", "); }
        query.push_str(&format!("username = ${}", i));
        binds.push(username.to_string());
        i += 1;
    }
    
    // Email update
    if let Some(email) = payload["email"].as_str() {
        if i > 1 { query.push_str(", "); }
        query.push_str(&format!("email = ${}", i));
        binds.push(email.to_string());
        i += 1;
    }
    
    // Password update
    if let Some(password) = payload["password"].as_str() {
        let password_hash = hash_password(password)?;
        if i > 1 { query.push_str(", "); }
        query.push_str(&format!("password_hash = ${}", i));
        binds.push(password_hash);
        i += 1;
    }
    
    // Update timestamp
    let now = OffsetDateTime::now_utc();
    if i > 1 { query.push_str(", "); }
    query.push_str(&format!("updated_at = ${}", i));
    binds.push(now.to_string());
    i += 1;
    
    // Add WHERE clause
    query.push_str(&format!(" WHERE id = ${}", i));
    
    // Execute query
    let mut db_query = sqlx::query(&query);
    
    // Add bindings
    for bind in binds {
        db_query = db_query.bind(bind);
    }
    
    // Add user_id binding
    db_query = db_query.bind(auth_user.user_id);
    
    // Execute query
    db_query.execute(&pool).await?;
    
    // Get updated user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&pool)
        .await?;
    
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
} 