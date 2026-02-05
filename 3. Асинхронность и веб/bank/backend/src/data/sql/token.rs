use crate::{
    domain::token::{self, RefreshToken, RefreshTokenRepository},
    infrastructure::{
        error::ErrorApi,
        security::{hash_token, REFRESH_TOKEN_DURATION},
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct RefreshTokenRow {
    refresh_token_hash: String,
    user_id: Uuid,
    expires_at: DateTime<Utc>,
}

impl From<RefreshTokenRow> for RefreshToken {
    fn from(row: RefreshTokenRow) -> Self {
        let token = token::get_token();
        RefreshToken::new(token, row.refresh_token_hash, row.user_id, row.expires_at)
    }
}

pub struct RefreshTokenSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl RefreshTokenRepository for RefreshTokenSQLRepo {
    async fn create(
        &mut self,
        refresh_token: String,
        user_id: Uuid,
    ) -> Result<RefreshToken, ErrorApi> {
        let expires_at = Utc::now() + REFRESH_TOKEN_DURATION;
        let token = token::factory::create(refresh_token, user_id, expires_at)?;

        let row = sqlx::query_as!(
            RefreshTokenRow,
            r#"
            INSERT INTO refresh_tokens (refresh_token_hash, user_id, expires_at)
            VALUES ($1, $2, $3)
            RETURNING refresh_token_hash, user_id, expires_at
            "#,
            token.refresh_token_hash(),
            token.user_id(),
            token.expires_at(),
        )
        .fetch_one(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        Ok(row.into())
    }

    async fn delete(&mut self, refresh_token: String) -> Result<RefreshToken, ErrorApi> {
        let Ok(refresh_token_hash) = hash_token(&refresh_token) else {
            return Err(ErrorApi::Inner("Hash error".to_string()));
        };
        let row = sqlx::query_as!(
            RefreshTokenRow,
            r#"
            DELETE FROM refresh_tokens
            WHERE refresh_token_hash = $1
            RETURNING refresh_token_hash, user_id, expires_at
            "#,
            refresh_token_hash,
        )
        .fetch_optional(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        row.ok_or(ErrorApi::NotFound("Refresh token not found".to_string()))
            .map(|r| r.into())
    }

    async fn get(&self, refresh_token: String) -> Result<RefreshToken, ErrorApi> {
        let Ok(refresh_token_hash) = hash_token(&refresh_token) else {
            return Err(ErrorApi::Inner("Hash error".to_string()));
        };
        let row = sqlx::query_as!(
            RefreshTokenRow,
            r#"
            SELECT refresh_token_hash, user_id, expires_at
            FROM refresh_tokens
            WHERE refresh_token_hash = $1
            "#,
            refresh_token_hash,
        )
        .fetch_optional(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        row.ok_or(ErrorApi::NotFound("Refresh token not found".to_string()))
            .map(|r| r.into())
    }
}
