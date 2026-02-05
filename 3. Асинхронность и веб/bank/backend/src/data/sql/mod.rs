use super::transaction::DBTransactionTrait;
use crate::infrastructure::error::ErrorApi;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
pub mod account;
pub mod course;
pub mod token;
pub mod transactions;
pub mod user;

pub struct DBTransactionSQL(Option<sqlx::Transaction<'static, sqlx::Postgres>>);

impl DBTransactionSQL {
    pub async fn new(pool: Arc<PgPool>) -> Result<Self, ErrorApi> {
        let tx = pool
            .begin()
            .await
            .map_err(|_| ErrorApi::DataBase("Begin transaction error".to_string()))?;
        Ok(Self(Some(tx)))
    }
}

#[async_trait]
impl DBTransactionTrait for DBTransactionSQL {
    async fn commit(&mut self) -> Result<(), ErrorApi> {
        let Some(tx) = self.0.take() else {
            return Err(ErrorApi::DataBase("Transaction not found".to_string()));
        };
        tx.commit()
            .await
            .map_err(|_| ErrorApi::DataBase("Commit error".to_string()))
    }

    async fn rollback(&mut self) -> Result<(), ErrorApi> {
        let Some(tx) = self.0.take() else {
            return Err(ErrorApi::DataBase("Transaction not found".to_string()));
        };
        tx.rollback()
            .await
            .map_err(|_| ErrorApi::DataBase("Rollback error".to_string()))
    }
}
