use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::errors::AppError;

// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);
    
    // Create Argon2 instance with default parameters
    let argon2 = Argon2::default();
    
    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
        .to_string();
    
    Ok(password_hash)
}

// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    // Parse the hash
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
    
    // Create Argon2 instance
    let argon2 = Argon2::default();
    
    // Verify the password
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
} 