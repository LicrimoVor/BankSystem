use anyhow::{anyhow, Result};
use argon2::password_hash::rand_core::OsRng;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub const REFRESH_TOKEN_DURATION: Duration = Duration::days(31 * 5);
const REFRESH_TOKEN_SALT: &str = "refresh_token";

pub fn hash_password(plain: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| anyhow!("argon2 hash error: {}", e))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(plain: &str, hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash).map_err(|e| anyhow!("argon2 hash error: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(plain.as_bytes(), &parsed).is_ok())
}

pub fn hash_token(plain: &str) -> Result<String> {
    let plain = format!("{}{}", plain, REFRESH_TOKEN_SALT);
    let hash = Sha256::digest(plain.as_bytes());
    Ok(hex::encode(hash))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

pub fn generate_jwt(secret: &str, user_id: Uuid) -> Result<String> {
    let now = Utc::now();
    let exp = now + Duration::minutes(15);

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

#[allow(dead_code)]
pub fn verify_jwt(secret: &str, token: &str) -> Result<Uuid> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    let uid = Uuid::parse_str(&data.claims.sub)?;
    Ok(uid)
}

pub fn generate_refresh_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}
