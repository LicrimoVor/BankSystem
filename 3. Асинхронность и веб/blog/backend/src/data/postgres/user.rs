use crate::{
    domain::user::{User, UserRepository, factory},
    infrastructure::{DATETIME_OFFSET, errors::ErrorBlog},
};
use chrono::FixedOffset;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    #[sea_orm(default_value = "now()")]
    pub created_at: chrono::DateTime<FixedOffset>,
    #[sea_orm(has_many)]
    pub posts: HasMany<super::post::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for User {
    fn from(row: Model) -> Self {
        factory::from_database(
            row.id,
            row.username,
            row.email,
            row.password_hash,
            row.created_at.to_utc(),
        )
    }
}

impl From<User> for ActiveModel {
    fn from(post: User) -> Self {
        ActiveModel {
            id: Set(post.id().clone()),
            username: Set(post.username().clone()),
            email: Set(post.email().clone()),
            password_hash: Set(post.password_hash().clone()),
            created_at: Set(post.created_at().with_timezone(&DATETIME_OFFSET)),
        }
    }
}

pub struct UserPostgresRepo(pub sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl UserRepository for UserPostgresRepo {
    async fn create(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, ErrorBlog> {
        let user = factory::create(username, email, password)?;
        let model = ActiveModel::from(user.clone());
        model.insert(&self.0).await?;
        Ok(user)
    }

    async fn delete(&self, user_id: Uuid) -> Result<User, ErrorBlog> {
        let Some(user) = Entity::find_by_id(user_id).one(&self.0).await? else {
            return Err(ErrorBlog::NotFound("User not found".to_string()));
        };

        user.clone().delete(&self.0).await?;

        Ok(user.into())
    }

    async fn update(&self, user_id: Uuid, user: User) -> Result<User, ErrorBlog> {
        if let None = Entity::find_by_id(user_id).one(&self.0).await? {
            return Err(ErrorBlog::NotFound("User not found".to_string()));
        };
        let active_model: ActiveModel = user.into();
        let user_model = active_model.update(&self.0).await?;
        Ok(user_model.into())
    }

    async fn get_by_id(&self, user_id: Uuid) -> Result<Option<User>, ErrorBlog> {
        let Some(user) = Entity::find_by_id(user_id).one(&self.0).await? else {
            return Ok(None);
        };
        Ok(Some(user.into()))
    }

    async fn get_by_email(&self, email: String) -> Result<Option<User>, ErrorBlog> {
        let Some(user) = Entity::find_by_email(email).one(&self.0).await? else {
            return Ok(None);
        };
        Ok(Some(user.into()))
    }

    async fn get_by_username(&self, username: String) -> Result<Option<User>, ErrorBlog> {
        let Some(user) = Entity::find_by_username(username).one(&self.0).await? else {
            return Ok(None);
        };
        Ok(Some(user.into()))
    }
}
