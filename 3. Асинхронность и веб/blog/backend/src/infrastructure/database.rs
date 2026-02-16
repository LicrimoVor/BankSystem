use crate::infrastructure::config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// Создает подключение к базе данных
pub async fn create_connection(config: &Config) -> anyhow::Result<DatabaseConnection> {
    let Some(url) = config.database_url.clone() else {
        return Err(anyhow::anyhow!("DATABASE_URL is not set"));
    };
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10));
    Database::connect(opt).await.map_err(|e| e.into())
}
