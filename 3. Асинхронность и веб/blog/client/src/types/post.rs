use crate::{dto, types::Error};

#[async_trait::async_trait]
pub trait PostClientTrait {
    async fn create(
        &mut self,
        title: &str,
        content: &str,
        img_base64: Option<&str>,
    ) -> Result<dto::Post, Error>;
    async fn update(
        &mut self,
        post_id: &str,
        title: Option<&str>,
        content: Option<&str>,
        img_base64: Option<&str>,
    ) -> Result<dto::Post, Error>;
    async fn delete(&mut self, post_id: &str) -> Result<(), Error>;
    async fn get_by_id(&mut self, post_id: &str) -> Result<dto::Post, Error>;
    async fn gets_by_author(&mut self, email: &str) -> Result<Vec<dto::Post>, Error>;
    async fn gets_me(&mut self) -> Result<Vec<dto::Post>, Error>;
}
