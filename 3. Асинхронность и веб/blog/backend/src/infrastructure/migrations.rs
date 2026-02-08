use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tracing::info;

/// Запускает миграции для базы данных
pub async fn run_migrations(connection: &DatabaseConnection) -> anyhow::Result<()> {
    Migrator::up(connection, None).await?;
    info!("Migrations applied successfully");
    Ok(())
}
