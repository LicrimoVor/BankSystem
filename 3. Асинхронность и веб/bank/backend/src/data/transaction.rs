use crate::infrastructure::error::ErrorApi;
use async_trait::async_trait;

#[async_trait]
pub trait DBTransaction: Send + Sync {
    async fn commit(&mut self) -> Result<(), ErrorApi>;
    async fn rollback(&mut self) -> Result<(), ErrorApi>;
}
