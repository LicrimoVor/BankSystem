use std::path::Path;
use tracing::{info, warn};

/// Конфигурация приложения
#[derive(Debug)]
pub struct Config {
    pub database_type: String,
    pub database_url: Option<String>,
    pub media_path: String,
    pub jwt_secret: String,
    pub port_api: u16,
    pub port_grpc: u16,
    pub host: String,
    pub cors_origin: Vec<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let mut database_type = std::env::var("DATABASE_TYPE").unwrap_or_else(|_| {
            warn!("DATABASE_TYPE is not set. Using MEMORY database.");
            "MEMORY".into()
        });
        let database_url = std::env::var("DATABASE_URL").ok();
        if database_url.is_none() && database_type != "MEMORY" {
            warn!("DATABASE_URL is not set. Using MEMORY database.");
            database_type = "MEMORY".into();
        }
        let media_path = std::env::var("MEDIA_PATH").unwrap_or_else(|_| {
            warn!("MEDIA_PATH is not set. Using default path: ./media");
            "./media".into()
        });
        Path::new(&media_path).exists().then(|| ()).ok_or_else(|| {
            anyhow::anyhow!(
                "Media path '{}' does not exist. Please create it or set MEDIA_PATH env variable to an existing directory.",
                media_path
            )
        })?;
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            warn!("JWT_SECRET is not set. Using random secret.");
            "secret".into()
        });
        let port_api = std::env::var("PORT_API")
            .unwrap_or_else(|_| {
                warn!("PORT_API is not set. Using default port: 8001");
                "8001".into()
            })
            .parse::<u16>()?;
        let port_grpc = std::env::var("PORT_GRPC")
            .unwrap_or_else(|_| {
                warn!("PORT_GRPC is not set. Using default port: 50051");
                "50051".into()
            })
            .parse::<u16>()?;
        let host = std::env::var("HOST").unwrap_or_else(|_| {
            warn!("HOST is not set. Using default host: 0.0.0.0");
            "0.0.0.0".into()
        });
        let cors_origin = std::env::var("CORS_ORIGIN")
            .map(|cors| cors.split(" ").map(|s| s.to_string()).collect())
            .unwrap_or(vec![
                format!("http://{}", host),
                "http://localhost:3000".into(),
            ]);

        info!("Successfully loaded configuration");
        Ok(Self {
            database_type,
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
