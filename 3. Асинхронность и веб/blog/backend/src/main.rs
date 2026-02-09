use std::sync::Arc;
pub(crate) mod data;
pub(crate) mod domain;
pub(crate) mod infrastructure;

/// Флаг, указывающий на режим разработки
/// (можно подтягивать из конфига, но для простоты оставим константой)
const IS_DEV: bool = true;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    infrastructure::logging::logging_init();

    let config = infrastructure::config::Config::from_env()?;
    let database = {
        if !IS_DEV {
            let connection = infrastructure::database::create_connection(&config).await?;
            infrastructure::migrations::run_migrations(&connection).await?;
            data::Database::Postgres(connection)
        } else {
            data::Database::Memory(Arc::new(infrastructure::state::State::new()))
        }
    };

    Ok(())
}
