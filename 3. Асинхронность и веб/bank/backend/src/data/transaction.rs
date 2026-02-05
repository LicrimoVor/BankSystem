use crate::{
    data::{sql::DBTransactionSQL, state::DBTransactionState},
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;

#[async_trait]
pub trait DBTransactionTrait: Send + Sync {
    async fn commit(&mut self) -> Result<(), ErrorApi>;
    async fn rollback(&mut self) -> Result<(), ErrorApi>;
}

pub enum DBTransaction {
    SQL(DBTransactionSQL),
    STATE(DBTransactionState),
}

impl DBTransaction {
    pub async fn commit(self) -> Result<(), ErrorApi> {
        match self {
            DBTransaction::SQL(mut tx) => tx.commit().await,
            DBTransaction::STATE(mut tx) => tx.commit().await,
        }
    }

    pub async fn rollback(self) -> Result<(), ErrorApi> {
        match self {
            DBTransaction::SQL(mut tx) => tx.rollback().await,
            DBTransaction::STATE(mut tx) => tx.rollback().await,
        }
    }
}
