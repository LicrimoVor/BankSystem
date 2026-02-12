use super::user::UserResponse;
use crate::domain::{auth::JwtToken, user::User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthLoginResponse {
    pub user: UserResponse,
    pub access_token: String,
}

impl AuthLoginResponse {
    pub fn new(user: User, jwt: JwtToken) -> AuthLoginResponse {
        AuthLoginResponse {
            access_token: jwt.0,
            user: UserResponse::new(user),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthLoginRequest {
    pub email: String,
    pub password: String,
}
