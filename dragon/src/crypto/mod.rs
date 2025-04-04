use std::env;
use argon2::{
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    Argon2
};
use base64::{Engine as _, engine::general_purpose};

// Simple encryption/decryption for credential passwords
// In a real-world scenario, you would use a proper encryption scheme with a secure key management system
// This is a simplified example using a key derived from the user's password hash

// Encrypt a password string using a key derived from the user's password hash
pub fn encrypt_password(password: &str, user_id: &str) -> Result<String, String> {
    // In a real app, you'd use a proper key derivation and encryption algorithm
    // This is a simplified example
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    // Use the user_id and app secret to create a deterministic encryption key
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let encryption_key = format!("{}:{}", user_id, jwt_secret);
    
    let derived_key = argon2.hash_password(encryption_key.as_bytes(), &salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?
        .to_string();
    
    // XOR the password with the derived key (simple encryption, not for production)
    let xor_bytes: Vec<u8> = password.bytes()
        .zip(derived_key.bytes().cycle())
        .map(|(p, k)| p ^ k)
        .collect();
    
    // Encode as base64 and include the salt
    let encrypted = general_purpose::STANDARD.encode(&xor_bytes);
    Ok(format!("{}:{}", salt, encrypted))
}

// Decrypt a password that was encrypted using encrypt_password
pub fn decrypt_password(encrypted: &str, user_id: &str) -> Result<String, String> {
    // Parse the salt and encrypted data
    let parts: Vec<&str> = encrypted.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid encrypted format".to_string());
    }
    
    let salt_str = parts[0];
    let encrypted_b64 = parts[1];
    
    // Parse the salt - updated to use from_b64 instead of new
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| format!("Invalid salt: {}", e))?;
    
    // Derive the same key as during encryption
    let argon2 = Argon2::default();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let encryption_key = format!("{}:{}", user_id, jwt_secret);
    
    let derived_key = argon2.hash_password(encryption_key.as_bytes(), &salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?
        .to_string();
    
    // Decode the base64 data
    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_b64)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    // XOR to decrypt
    let decrypted_bytes: Vec<u8> = encrypted_bytes.iter()
        .zip(derived_key.bytes().cycle())
        .map(|(c, k)| c ^ k)
        .collect();
    
    // Convert back to string
    String::from_utf8(decrypted_bytes)
        .map_err(|e| format!("Failed to convert to string: {}", e))
} 