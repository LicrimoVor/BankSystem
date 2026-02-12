use crate::{dto, types::Error};

#[async_trait::async_trait]
pub trait AuthClientTrait {
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<dto::User, Error>;
    async fn login(&mut self, email: &str, password: &str) -> Result<dto::User, Error>;
    async fn logout(&mut self) -> Result<(), Error>;
}
