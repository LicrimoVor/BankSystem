use crate::{
    domain::user::{User, UserRepository, factory},
    infrastructure::errors::ErrorBlog,
};
use uuid::Uuid;

struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        factory::from_database(
            row.id,
            row.username,
            row.email,
            row.password_hash,
            row.created_at,
        )
    }
}

pub struct UserPostgresRepo(pub sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl UserRepository for UserPostgresRepo {
    async fn create(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, ErrorBlog> {
        // Реализовать создание пользователя в базе данных PostgreSQL
        unimplemented!()
    }

    async fn delete(&self, user_id: Uuid) -> Result<User, ErrorBlog> {
        // Реализовать удаление пользователя из базы данных PostgreSQL
        unimplemented!()
    }

    async fn update(&self, user_id: Uuid, user: User) -> Result<User, ErrorBlog> {
        // Реализовать обновление пользователя в базе данных PostgreSQL
        unimplemented!()
    }

    async fn get_by_id(&self, user_id: Uuid) -> Result<Option<User>, ErrorBlog> {
        // Реализовать получение пользователя по ID из базы данных PostgreSQL
        unimplemented!()
    }

    async fn get_by_email(&self, email: String) -> Result<Option<User>, ErrorBlog> {
        // Реализовать получение пользователя по email из базы данных PostgreSQL
        unimplemented!()
    }

    async fn get_by_username(&self, username: String) -> Result<Option<User>, ErrorBlog> {
        // Реализовать получение пользователя по username из базы данных PostgreSQL
        unimplemented!()
    }
}
