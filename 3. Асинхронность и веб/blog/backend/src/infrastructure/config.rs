use std::path::Path;

use tracing::info;

/// Конфигурация приложения
#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub media_path: String,
    pub jwt_secret: String,
    pub port_api: u16,
    pub port_grpc: u16,
    pub host: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let media_path = std::env::var("MEDIA_PATH").unwrap_or_else(|_| "./media".into());
        Path::new(&media_path).exists().then(|| ()).ok_or_else(|| {
            anyhow::anyhow!(
                "Media path '{}' does not exist. Please create it or set MEDIA_PATH env variable to an existing directory.",
                media_path
            )
        })?;
        let jwt_secret = std::env::var("JWT_SECRET")?;
        let port_api = std::env::var("PORT_API")?.parse::<u16>()?;
        let port_grpc = std::env::var("PORT_GRPC")?.parse::<u16>()?;
        let host = std::env::var("HOST")?;
        let cors_origin =
            std::env::var("CORS_ORIGIN").unwrap_or_else(|_| format!("http://{}", host));

        info!("Successfully loaded configuration");
        Ok(Self {
            database_url,
            media_path,
            jwt_secret,
            port_api,
            port_grpc,
            host,
            cors_origin,
        })
    }
}
