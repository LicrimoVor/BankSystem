use crate::types::Error;

#[async_trait::async_trait]
pub trait GeneralClientTrait {
    async fn health(&mut self) -> Result<String, Error>;
    async fn ping(&mut self) -> Result<String, Error>;
}
