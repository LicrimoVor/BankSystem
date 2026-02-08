use crate::infrastructure::errors::ErrorBlog;
use getset::{Getters, Setters};
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Clone, Getters, Setters)]
pub struct User {
    #[getset(get = "pub")]
    id: Uuid,
    #[getset(get = "pub", set = "pub")]
    username: String,
    #[getset(get = "pub", set = "pub")]
    email: String,
    #[getset(get = "pub")]
    password_hash: String,
    #[getset(get = "pub")]
    created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn change_password(&mut self, new_password: String) -> Result<(), ErrorBlog> {
        warn!("!!!");
        let new_password_hash = new_password;
        self.password_hash = new_password_hash;
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, ErrorBlog>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<User, ErrorBlog>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, ErrorBlog>;
    async fn get_user_by_email(&self, email: String) -> Result<Option<User>, ErrorBlog>;
    async fn get_user_by_username(&self, username: String) -> Result<Option<User>, ErrorBlog>;
}

pub mod factory {
    use super::*;

    pub fn create(username: String, email: String, password: String) -> Result<User, ErrorBlog> {
        let id = Uuid::new_v4();
        warn!("!!!");
        let password_hash = password;
        Ok(User {
            id,
            username,
            email,
            password_hash,
            created_at: chrono::Utc::now(),
        })
    }
}
