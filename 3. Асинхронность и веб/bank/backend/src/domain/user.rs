use async_trait::async_trait;
use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{impl_constructor, infrastructure::error::ErrorApi};

const EMAIL_REGEX: &str = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+.[A-Za-z]{2,}$";
const MIN_PASSWORD_LEN: usize = 8;

#[derive(Debug, Serialize, Getters, Setters, Clone, FromRow)]
pub struct User {
    #[getset(get = "pub")]
    id: Uuid,

    #[getset(get = "pub")]
    created_at: DateTime<Utc>,

    #[getset(get = "pub", set = "pub")]
    email: String,

    #[getset(get = "pub", set = "pub")]
    password_hash: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&mut self, email: String, password_hash: String) -> Result<User, ErrorApi>;
    async fn update(&mut self, user: &User) -> Result<(), ErrorApi>;
    async fn delete(&mut self, user: &User) -> Result<(), ErrorApi>;
    async fn get_by_email(&self, email: String) -> Option<User>;
    async fn get_by_id(&self, id: Uuid) -> Option<User>;
}

// impl_constructor!(token: UserToken, User, (id: Uuid, created_at: DateTime<Utc>, email: String, password_hash: String));
// impl_constructor!(factory: User, (id: Uuid, created_at: DateTime<Utc>, email: String, password_hash: String));

pub mod factory {
    use super::*;
    use crate::infrastructure::security::hash_password;
    use regex::Regex;

    pub fn create(
        id: Uuid,
        created_at: DateTime<Utc>,
        email: String,
        password: String,
    ) -> Result<User, ErrorApi> {
        if password.len() < MIN_PASSWORD_LEN {
            return Err(ErrorApi::Validation(
                "Password must be at least 8 characters".to_string(),
            ));
        }
        let Ok(regex) = Regex::new(EMAIL_REGEX) else {
            return Err(ErrorApi::Inner("Invalid regex".to_string()));
        };
        if !regex.is_match(&email) {
            return Err(ErrorApi::Validation("Invalid email format".to_string()));
        };
        let Ok(password_hash) = hash_password(&password) else {
            return Err(ErrorApi::Inner("Hash error".to_string()));
        };

        Ok(User {
            id,
            created_at,
            email,
            password_hash,
        })
    }
}
