use crate::{
    domain::post::{Post, PostRepository, factory},
    infrastructure::errors::ErrorBlog,
};
use uuid::Uuid;

struct PostRow {
    id: Uuid,
    title: String,
    content: String,
    img_path: Option<String>,
    author_id: Uuid,
    updated_at: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<PostRow> for Post {
    fn from(row: PostRow) -> Self {
        factory::from_database(
            row.id,
            row.title,
            row.content,
            row.author_id,
            row.img_path,
            row.created_at,
            row.updated_at,
        )
    }
}

pub struct PostPostgresRepo(pub sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl PostRepository for PostPostgresRepo {
    async fn create(
        &self,
        title: String,
        content: String,
        author_id: Uuid,
        img_path: Option<String>,
    ) -> Result<Post, ErrorBlog> {
        unimplemented!()
    }

    async fn update(&self, post_id: Uuid, post: Post) -> Result<Post, ErrorBlog> {
        unimplemented!()
    }

    async fn delete(&self, post_id: Uuid) -> Result<Post, ErrorBlog> {
        unimplemented!()
    }

    async fn get_by_id(&self, post_id: Uuid) -> Result<Option<Post>, ErrorBlog> {
        unimplemented!()
    }

    async fn gets_by_author(&self, author_id: Uuid) -> Result<Vec<Post>, ErrorBlog> {
        unimplemented!()
    }
}
