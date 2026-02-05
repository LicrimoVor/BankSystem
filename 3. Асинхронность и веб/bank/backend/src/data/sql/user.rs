use crate::{
    domain::user::{self, User, UserRepository},
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{prelude::FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(FromRow, Debug)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    created_at: chrono::DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        let token = user::get_token();
        User::new(token, row.id, row.created_at, row.email, row.password_hash)
    }
}

pub struct UserSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl UserRepository for UserSQLRepo {
    async fn create(&mut self, email: String, password_hash: String) -> Result<User, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        let user = user::factory::create(id, created_at, email.clone(), password_hash.clone())?;

        let res = sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            user.id(),
            user.email(),
            user.password_hash(),
            user.created_at()
        )
        .execute(self.0.as_ref())
        .await;

        match res {
            Ok(_) => Ok(user),
            Err(e) => {
                // уникальность email
                if let Some(db_err) = e.as_database_error() {
                    if db_err.constraint() == Some("users_email_key") {
                        return Err(ErrorApi::DataBase("Email already exists".into()));
                    }
                }
                Err(ErrorApi::DataBase(e.to_string()))
            }
        }
    }

    async fn update(&mut self, user: &User) -> Result<(), ErrorApi> {
        let res = sqlx::query!(
            r#"
            UPDATE users
            SET email = $2,
                password_hash = $3
            WHERE id = $1
            "#,
            user.id(),
            user.email(),
            user.password_hash(),
        )
        .execute(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        if res.rows_affected() == 0 {
            return Err(ErrorApi::DataBase("User not found".into()));
        }

        Ok(())
    }

    async fn delete(&mut self, user: &User) -> Result<(), ErrorApi> {
        let res = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            user.id()
        )
        .execute(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        if res.rows_affected() == 0 {
            return Err(ErrorApi::DataBase("User not found".into()));
        }

        Ok(())
    }

    async fn get_by_email(&self, email: String) -> Option<User> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self.0.as_ref())
        .await
        .ok()??;

        Some(User::from(user_row))
    }

    async fn get_by_id(&self, id: Uuid) -> Option<User> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.0.as_ref())
        .await
        .ok()??;

        Some(User::from(user_row))
    }
}
