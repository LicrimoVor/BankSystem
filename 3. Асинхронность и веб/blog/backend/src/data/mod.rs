pub mod memory;
pub mod postgres;
use self::memory::user::UserStateRepo;
use self::postgres::user::UserPostgresRepo;
use crate::domain::user::UserRepository;
use crate::infrastructure::state::State;
use std::sync::Arc;

/// База данных приложения
/// Может быть как Postgres, так и in-memory
pub enum Database {
    Postgres(sea_orm::DatabaseConnection),
    Memory(Arc<State>),
}

/// Макрос для генерации методов получения сервисов из базы данных  
macro_rules! impl_get_service {
    ($name:ident, $repo_trait:ident, $repo_postgres:ident, $repo_memory:ident) => {
        pub async fn $name(self: Arc<Self>) -> Box<dyn $repo_trait> {
            match self.as_ref() {
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
    impl_get_service!(
        get_user_service,
        UserRepository,
        UserPostgresRepo,
        UserStateRepo
    );
    // impl_get_service!(get_post_service, PostRepository, PostRepositoryPostgres, PostRepositoryMemory);
}
