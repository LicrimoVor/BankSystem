use crate::{
    domain::auth::{AuthRepository, RefreshToken},
    infrastructure::errors::ErrorBlog,
};
use sea_orm::{ActiveValue::Set, entity::prelude::*};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "auth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub refresh_token: String,
    #[sea_orm(column_name = "user_id")]
    pub user_id: Uuid,
}

impl ActiveModelBehavior for ActiveModel {}

pub struct AuthPostgresRepo(pub sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl AuthRepository for AuthPostgresRepo {
    /// Создать refresh token и сохранить в БД
    async fn create_refresh_token(&mut self, user_id: Uuid) -> Result<RefreshToken, ErrorBlog> {
        let refresh_token = RefreshToken::generate();
        let model = ActiveModel {
            refresh_token: Set(refresh_token.clone().0),
            user_id: Set(user_id),
        };

        model
            .insert(&self.0)
            .await
            .map_err(|e| ErrorBlog::Database(e.to_string()))?;

        Ok(refresh_token)
    }

    /// Получить user_id по токену
    async fn get_refresh_token(&self, token: RefreshToken) -> Option<Uuid> {
        let Ok(Some(a)) = Entity::find_by_id(token.0.clone()).one(&self.0).await else {
            return None;
        };
        Some(a.user_id)
    }

    /// Удалить токен из БД
    async fn delete_refresh_token(&mut self, token: RefreshToken) -> Result<Uuid, ErrorBlog> {
        if let Some(model) = Entity::find_by_id(token.0.clone()).one(&self.0).await? {
            let user_id = model.user_id;
            let active_model: ActiveModel = model.into();
            active_model
                .delete(&self.0)
                .await
                .map_err(|e| ErrorBlog::Database(e.to_string()))?;
            Ok(user_id)
        } else {
            Err(ErrorBlog::NotFound("Refresh token not found".to_string()))
        }
    }
}
