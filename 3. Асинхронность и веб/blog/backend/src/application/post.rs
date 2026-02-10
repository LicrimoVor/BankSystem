use crate::{
    data::Database,
    domain::post::Post,
    infrastructure::{config::Config, errors::ErrorBlog},
    utils::save_img::save_image,
};
use std::sync::Arc;
use uuid::Uuid;

struct PostService(pub Arc<Database>);

impl PostService {
    pub async fn create(
        &self,
        config: Arc<Config>,
        title: String,
        content: String,
        author_id: Uuid,
        image: Option<String>,
    ) -> Result<Post, ErrorBlog> {
        let post_repo = self.0.get_post_repo().await;
        if let Some(image) = image {
            let img_path = save_image(config, image).await?;
            post_repo
                .create(title, content, author_id, Some(img_path))
                .await
        } else {
            post_repo.create(title, content, author_id, None).await
        }
    }

    pub async fn get_by_id(&self, post_id: Uuid) -> Result<Post, ErrorBlog> {
        let post_repo = self.0.get_post_repo().await;
        post_repo
            .get_by_id(post_id)
            .await?
            .ok_or_else(|| ErrorBlog::NotFound(format!("Post with id {} not found", post_id)))
    }

    pub async fn gets_by_author(&self, author_id: Uuid) -> Result<Vec<Post>, ErrorBlog> {
        let post_repo = self.0.get_post_repo().await;
        post_repo.gets_by_author(author_id).await
    }

    pub async fn update(
        &self,
        post_id: Uuid,
        config: Arc<Config>,
        title: Option<String>,
        content: Option<String>,
        image: Option<String>,
    ) -> Result<Post, ErrorBlog> {
        if (None, None, None) == (title.clone(), content.clone(), image.clone()) {
            return Err(ErrorBlog::Argument(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let post_repo = self.0.get_post_repo().await;
        let mut post = post_repo
            .get_by_id(post_id)
            .await?
            .ok_or_else(|| ErrorBlog::NotFound(format!("Post with id {} not found", post_id)))?;
        if let Some(title) = title {
            post.set_title(title);
        }
        if let Some(content) = content {
            post.set_content(content);
        }
        if let Some(image) = image {
            let img_path = save_image(config, image).await?;
            post.set_img_path(Some(img_path));
        }
        post_repo.update(post_id, post.clone()).await
    }

    pub async fn delete(&self, post_id: Uuid) -> Result<Post, ErrorBlog> {
        let post_repo = self.0.get_post_repo().await;
        post_repo.delete(post_id).await
    }
}
