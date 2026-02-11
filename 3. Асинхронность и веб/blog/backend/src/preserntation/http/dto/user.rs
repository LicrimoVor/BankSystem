use serde::{Deserialize, Serialize};

use crate::domain::user::User;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}

impl UserResponse {
    pub fn new(user: User) -> UserResponse {
        UserResponse {
            username: user.username().clone(),
            email: user.email().clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
