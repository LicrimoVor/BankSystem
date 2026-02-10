use serde::{Deserialize, Serialize};

use crate::domain::{auth::JwtToken, user::User};

#[derive(Debug, Deserialize)]
pub struct AuthRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthLoginResponse {
    pub username: String,
    pub email: String,
    pub access_token: String,
}

impl AuthLoginResponse {
    pub fn new(user: User, jwt: JwtToken) -> AuthLoginResponse {
        AuthLoginResponse {
            access_token: jwt.0,
            username: user.username().clone(),
            email: user.email().clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthLoginRequest {
    pub email: String,
    pub password: String,
}
