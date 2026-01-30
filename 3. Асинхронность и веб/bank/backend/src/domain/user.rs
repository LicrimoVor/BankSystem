use async_trait::async_trait;
use chrono::{DateTime, Utc};
use getset::Getters;
use serde::Serialize;
use uuid::Uuid;

use crate::{impl_constructor, infrastructure::error::ErrorApi};

#[derive(Debug, Serialize, Clone, Getters)]
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
impl_constructor!(factory: User, (id: Uuid, created_at: DateTime<Utc>, email: String, password_hash: String));
