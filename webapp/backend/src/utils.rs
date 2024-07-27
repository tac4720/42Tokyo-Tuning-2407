use sha1::{Sha1, Digest};
use rand::Rng;
use std::fmt::Write; // For `write!` macro

use crate::errors::AppError;

pub fn generate_session_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..30)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            chars[idx] as char
        })
        .collect();
    token
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let password_bytes = password.as_bytes();
    
    // Use SHA-1 for hashing
    let mut hasher = Sha1::new();
    hasher.update(password_bytes);
    let result = hasher.finalize();
    let mut hash_string = String::new();
    for byte in result {
        write!(&mut hash_string, "{:02x}", byte).unwrap();
    }
    Ok(hash_string)
}

pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    let input_password_bytes = input_password.as_bytes();
    
    // Hash the input password and compare
    let input_password_hash = hash_password(input_password)?;
    Ok(hashed_password == input_password_hash)
}
