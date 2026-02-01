use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
}

impl From<crate::domain::user::User> for UserDto {
    fn from(value: crate::domain::user::User) -> Self {
        Self {
            id: value.id().clone(),
            email: value.email().clone(),
        }
    }
}

#[derive(Serialize)]
pub struct UserLoginDto {
    pub id: Uuid,
    pub email: String,
    pub access_token: String,
}
