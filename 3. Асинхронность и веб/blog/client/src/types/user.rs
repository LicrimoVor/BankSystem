use crate::{dto, types::Error};

#[async_trait::async_trait]
pub trait UserClientTrait {
    async fn me(&mut self) -> Result<dto::User, Error>;
    async fn update(
        &mut self,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<dto::User, Error>;

    async fn delete(&mut self) -> Result<(), Error>;
    async fn get_by_email(&mut self, email: &str) -> Result<dto::User, Error>;
}
