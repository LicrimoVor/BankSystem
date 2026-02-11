use crate::{
    domain::post::{Post, PostRepository, factory},
    infrastructure::{DATETIME_OFFSET, errors::ErrorBlog},
};
use chrono::FixedOffset;
use sea_orm::{ActiveValue::Set, Insert, entity::prelude::*};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub img_path: Option<String>,
    pub author_id: Uuid,
    #[sea_orm(default_value = "now()")]
    pub updated_at: chrono::DateTime<FixedOffset>,
    #[sea_orm(default_value = "now()")]
    pub created_at: chrono::DateTime<FixedOffset>,
    #[sea_orm(belongs_to, from = "author_id", to = "id")]
    pub author: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Post {
    fn from(row: Model) -> Self {
        factory::from_database(
            row.id,
            row.title,
            row.content,
            row.author_id,
            row.img_path,
            row.created_at.to_utc(),
            row.updated_at.to_utc(),
        )
    }
}

impl From<Post> for ActiveModel {
    fn from(post: Post) -> Self {
        ActiveModel {
            id: Set(post.id().clone()),
            title: Set(post.title().clone()),
            content: Set(post.content().clone()),
            img_path: Set(post.img_path().clone()),
            author_id: Set(post.author_id().clone()),
            updated_at: Set(post.updated_at().with_timezone(&DATETIME_OFFSET)),
            created_at: Set(post.created_at().with_timezone(&DATETIME_OFFSET)),
        }
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
        let post = factory::create(title, content, author_id, img_path)?;
        Insert::one(ActiveModel::from(post.clone()))
            .exec(&self.0)
            .await?;

        Ok(post)
    }

    async fn update(&self, post_id: Uuid, post: Post) -> Result<Post, ErrorBlog> {
        if let None = Entity::find_by_id(post_id).one(&self.0).await? {
            return Err(ErrorBlog::NotFound("Post not found".to_string()));
        };

        let active_model: ActiveModel = post.into();
        let post_model = active_model.update(&self.0).await?;

        Ok(post_model.into())
    }

    async fn delete(&self, post_id: Uuid) -> Result<Post, ErrorBlog> {
        let Some(model) = Entity::find_by_id(post_id).one(&self.0).await? else {
            return Err(ErrorBlog::NotFound("Post not found".to_string()));
        };

        let post = model.clone().into();
        model.delete(&self.0).await?;
        Ok(post)
    }

    async fn get_by_id(&self, post_id: Uuid) -> Result<Option<Post>, ErrorBlog> {
        Entity::find_by_id(post_id)
            .one(&self.0)
            .await
            .map(|opt_m| opt_m.map(Post::from))
            .map_err(ErrorBlog::from)
    }

    async fn gets_by_author(&self, author_id: Uuid) -> Result<Vec<Post>, ErrorBlog> {
        Ok(super::user::Entity::find_by_id(author_id)
            .find_also_related(super::post::Entity)
            .all(&self.0)
            .await?
            .into_iter()
            .map(|(_, b)| b.map(Post::from))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect())
    }
}
