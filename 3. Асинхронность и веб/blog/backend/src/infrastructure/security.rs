use crate::infrastructure::errors::ErrorBlog;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use rand::{RngExt, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const REFRESH_TOKEN_DURATION: Duration = Duration::days(31 * 5);
pub const JWT_TOKEN_DURATION: Duration = Duration::hours(3);
const HASH_SALT: &str = "salt_for_hashing";

pub fn generate_password_hash(password: &str) -> Result<String, ErrorBlog> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ErrorBlog::Internal(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, ErrorBlog> {
    let parsed_hash =
        PasswordHash::new(&password_hash).map_err(|e| ErrorBlog::Internal(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_hash(plain: &str) -> Result<String, ErrorBlog> {
    let plain = format!("{}{}", plain, HASH_SALT);
    let hash = Sha256::digest(plain.as_bytes());
    Ok(hex::encode(hash))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_jwt_token(sercret: &str, user_id: &str) -> Result<String, ErrorBlog> {
    let now = Utc::now();
    let exp = now + JWT_TOKEN_DURATION;
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(sercret.as_bytes()),
    );
    token.map_err(|e| ErrorBlog::Internal(e.to_string()))
}

pub fn verify_jwt_token(token: &str) -> Result<Claims, ErrorBlog> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(HASH_SALT.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|e| ErrorBlog::Internal(e.to_string()))?;
    Ok(data.claims)
}

pub fn generate_refresh_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}
