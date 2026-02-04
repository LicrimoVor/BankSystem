use super::transaction::DBTransaction;
use crate::infrastructure::error::ErrorApi;
use async_trait::async_trait;
use tracing::info;
pub mod account;
pub mod token;
pub mod transactions;
pub mod user;

pub struct DBTransactionState;

impl DBTransactionState {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DBTransaction for DBTransactionState {
    async fn commit(&mut self) -> Result<(), ErrorApi> {
        info!("transaction commit");
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), ErrorApi> {
        info!("transaction rollback");
        Ok(())
    }
}
