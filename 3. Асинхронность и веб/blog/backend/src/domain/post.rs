use crate::infrastructure::errors::ErrorBlog;
use getset::{Getters, Setters};
use uuid::Uuid;

/// Пост в блоге пользователя
#[derive(Debug, Clone, Getters, Setters)]
pub struct Post {
    #[getset(get = "pub")]
    id: Uuid,
    #[getset(get = "pub", set = "pub")]
    title: String,
    #[getset(get = "pub", set = "pub")]
    content: String,
    #[getset(get = "pub")]
    img_path: Option<String>,
    #[getset(get = "pub")]
    author_id: Uuid,
    #[getset(get = "pub", set = "pub")]
    updated_at: chrono::DateTime<chrono::Utc>,
    #[getset(get = "pub")]
    created_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait::async_trait]
pub trait PostRepository {
    async fn create_post(
        &self,
        title: String,
        content: String,
        author_id: Uuid,
    ) -> Result<Post, ErrorBlog>;
    async fn update_post(&self, post_id: Uuid, post: Post) -> Result<Post, ErrorBlog>;
    async fn get_post_by_id(&self, post_id: Uuid) -> Result<Option<Post>, ErrorBlog>;
    async fn get_posts_by_author(&self, author_id: Uuid) -> Result<Vec<Post>, ErrorBlog>;
}

pub mod factory {
    use super::*;

    pub fn create(
        title: String,
        content: String,
        author_id: Uuid,
        img_path: Option<String>,
    ) -> Result<Post, ErrorBlog> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        Ok(Post {
            id,
            title,
            img_path,
            content,
            author_id,
            updated_at: now.clone(),
            created_at: now,
        })
    }

    /// Использовать только для создания объекта из данных, полученных из базы данных
    pub fn from_database(
        id: Uuid,
        title: String,
        content: String,
        author_id: Uuid,
        img_path: Option<String>,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Post {
        Post {
            id,
            title,
            content,
            author_id,
            img_path,
            created_at,
            updated_at,
        }
    }
}
