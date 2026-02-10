mod memory;
mod postgres;
pub mod transaction;
use self::{
    memory::{auth::AuthStateRepo, post::PostStateRepo, user::UserStateRepo},
    postgres::{auth::AuthPostgresRepo, post::PostPostgresRepo, user::UserPostgresRepo},
};
use crate::domain::{auth::AuthRepository, post::PostRepository, user::UserRepository};
use crate::infrastructure::state::State;
use std::sync::Arc;

/// База данных приложения
/// Может быть как Postgres, так и in-memory
pub enum Database {
    Postgres(sea_orm::DatabaseConnection),
    Memory(Arc<State>),
}

/// Макрос для генерации методов получения сервисов из базы данных  
macro_rules! impl_get_repo {
    ($name:ident, $repo_trait:ident, $repo_postgres:ident, $repo_memory:ident) => {
        pub async fn $name(self: &Arc<Self>) -> Box<dyn $repo_trait> {
            match self.clone().as_ref() {
                Database::Postgres(connection) => {
                    Box::new($repo_postgres(connection.clone())) as Box<dyn $repo_trait>
                }
                Database::Memory(state) => {
                    Box::new($repo_memory(state.clone())) as Box<dyn $repo_trait>
                }
            }
        }
    };
}

impl Database {
    impl_get_repo!(
        get_user_repo,
        UserRepository,
        UserPostgresRepo,
        UserStateRepo
    );
    impl_get_repo!(
        get_post_repo,
        PostRepository,
        PostPostgresRepo,
        PostStateRepo
    );
    impl_get_repo!(
        get_auth_repo,
        AuthRepository,
        AuthPostgresRepo,
        AuthStateRepo
    );
}
