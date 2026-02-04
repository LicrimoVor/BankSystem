use crate::{
    domain::{
        account::Account,
        transaction::{Transaction, TransactionRepository},
    },
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TransactionSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl TransactionRepository for TransactionSQLRepo {
    async fn create_deposit(&mut self, amount: f64, to: &Account) -> Result<Transaction, ErrorApi> {
        todo!()
    }
    async fn create_withdrawal(
        &mut self,
        amount: f64,
        from: &Account,
    ) -> Result<Transaction, ErrorApi> {
        todo!()
    }
    async fn create_transfer(
        &mut self,
        amount: f64,
        from: &Account,
        to: &Account,
    ) -> Result<Transaction, ErrorApi> {
        todo!()
    }
    async fn delete(&mut self, transaction: &Transaction) -> Result<(), ErrorApi> {
        todo!()
    }
    async fn get_by_id(&self, id: Uuid) -> Option<Transaction> {
        todo!()
    }
    async fn gets_by_account(&self, account: &Account) -> Option<Vec<Transaction>> {
        todo!()
    }
}
