use data_encoding::BASE32;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::AppError;

// Character set for TOTP codes (digits only)
const CHARSET: &[u8] = b"0123456789";

// Generate a random TOTP secret
pub fn generate_totp_secret(app_name: &str, account_name: &str) -> Result<(String, String), AppError> {
    // Generate a random 20-byte secret
    let mut rng = rand::thread_rng();
    let mut secret_bytes = [0u8; 20];
    for byte in secret_bytes.iter_mut() {
        *byte = rng.gen();
    }
    
    // Encode in Base32 for human readability
    let secret = BASE32.encode(&secret_bytes);
    
    // Create provisioning URI for QR code
    let uri = format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}",
        app_name, account_name, secret, app_name
    );
    
    Ok((secret, uri))
}

// Generate a TOTP code
pub fn generate_totp(secret: &str, step: u64, digits: usize) -> Result<String, AppError> {
    // Decode the secret from Base32
    let secret_bytes = BASE32.decode(secret.as_bytes())
        .map_err(|_| AppError::Internal("Invalid TOTP secret".to_string()))?;
    
    // Get current timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::Internal("System time error".to_string()))?
        .as_secs();
    
    // Calculate counter value (number of steps)
    let counter = now / step;
    
    // Convert counter to big-endian bytes
    let counter_bytes = counter.to_be_bytes();
    
    // Create HMAC-SHA1 instance
    let mut mac = Hmac::<Sha1>::new_from_slice(&secret_bytes)
        .map_err(|_| AppError::Internal("HMAC creation error".to_string()))?;
    
    // Add counter to HMAC
    mac.update(&counter_bytes);
    
    // Get HMAC result
    let result = mac.finalize().into_bytes();
    
    // Dynamic truncation
    let offset = (result[19] & 0xf) as usize;
    let binary = ((result[offset] & 0x7f) as u32) << 24
        | (result[offset + 1] as u32) << 16
        | (result[offset + 2] as u32) << 8
        | (result[offset + 3] as u32);
    
    // Modulo to get the specified number of digits
    let modulo = 10u32.pow(digits as u32);
    let code = binary % modulo;
    
    // Convert to string and pad with leading zeros if needed
    let code_str = format!("{:0width$}", code, width = digits);
    
    Ok(code_str)
}

// Verify a TOTP code
pub fn verify_totp(secret: &str, code: &str, step: u64, digits: usize) -> Result<bool, AppError> {
    let generated = generate_totp(secret, step, digits)?;
    
    // Simple string comparison
    Ok(generated == code)
} 