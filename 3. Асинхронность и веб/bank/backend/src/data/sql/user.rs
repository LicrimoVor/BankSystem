use crate::{
    domain::user::{self, User, UserRepository},
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl UserRepository for UserSQLRepo {
    async fn create(&mut self, email: String, password_hash: String) -> Result<User, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let user = user::factory::create(id, created_at, email, password_hash)?;
        sqlx::query!("",);

        todo!()
    }

    async fn update(&mut self, user: &User) -> Result<(), ErrorApi> {
        todo!()
    }

    async fn delete(&mut self, user: &User) -> Result<(), ErrorApi> {
        todo!()
    }

    async fn get_by_email(&self, email: String) -> Option<User> {
        todo!()
    }

    async fn get_by_id(&self, id: Uuid) -> Option<User> {
        todo!()
    }
}
