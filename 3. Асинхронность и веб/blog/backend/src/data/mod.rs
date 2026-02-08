use crate::infrastructure::state::State;

/// База данных приложения
/// Может быть как Postgres, так и in-memory
pub enum Database {
    Postgres(sea_orm::DatabaseConnection),
    Memory(State),
}

/// Макрос для генерации методов получения сервисов из базы данных  
macro_rules! impl_get_service {
    ($name:ident, $repo_trait:ident, $repo_postgres:ident, $repo_memory:ident) => {
        pub async fn $name(self: Arc<Self>) -> Result<Box<dyn $repo_trait>, anyhow::Error> {
            match self.as_ref() {
                Database::Postgres(connection) => {
                    let repo =
                        crate::infrastructure::$repo_postgres::new(connection.clone()).await?;
                    Ok(crate::domain::$service::new(repo))
                }
                Database::Memory(state) => {
                    let repo = crate::infrastructure::$repo_memory::new(state.clone());
                    Ok(crate::domain::$service::new(repo))
                }
            }
        }
    };
}
