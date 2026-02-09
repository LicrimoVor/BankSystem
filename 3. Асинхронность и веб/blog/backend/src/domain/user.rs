use crate::infrastructure::{errors::ErrorBlog, security::generate_password_hash};
use getset::{Getters, Setters};
use uuid::Uuid;

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 128;
const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

/// Пользователь
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
        if new_password.len() < MIN_PASSWORD_LENGTH {
            return Err(ErrorBlog::Validation(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        if new_password.len() > MAX_PASSWORD_LENGTH {
            return Err(ErrorBlog::Validation(
                "Password must be at most 128 characters long".to_string(),
            ));
        }
        self.password_hash = generate_password_hash(new_password.as_str())?;
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, ErrorBlog>;
    async fn update(&self, user_id: Uuid, user: User) -> Result<User, ErrorBlog>;
    async fn delete(&self, user_id: Uuid) -> Result<User, ErrorBlog>;
    async fn get_by_id(&self, user_id: Uuid) -> Result<Option<User>, ErrorBlog>;
    async fn get_by_email(&self, email: String) -> Result<Option<User>, ErrorBlog>;
    async fn get_by_username(&self, username: String) -> Result<Option<User>, ErrorBlog>;
}

pub mod factory {
    use super::*;

    pub fn create(username: String, email: String, password: String) -> Result<User, ErrorBlog> {
        let id = Uuid::new_v4();
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(ErrorBlog::Validation(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(ErrorBlog::Validation(
                "Password must be at most 128 characters long".to_string(),
            ));
        }
        if !regex::Regex::new(EMAIL_REGEX)
            .unwrap()
            .is_match(email.as_str())
        {
            return Err(ErrorBlog::Validation("Invalid email format".to_string()));
        }
        let password_hash = generate_password_hash(password.as_str())?;
        Ok(User {
            id,
            username,
            email,
            password_hash,
            created_at: chrono::Utc::now(),
        })
    }

    /// Использовать только для создания объекта из данных, полученных из базы данных
    pub fn from_database(
        id: Uuid,
        username: String,
        email: String,
        password_hash: String,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> User {
        User {
            id,
            username,
            email,
            password_hash,
            created_at,
        }
    }
}
