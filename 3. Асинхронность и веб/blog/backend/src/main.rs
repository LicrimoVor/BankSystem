use crate::{
    data::Database,
    infrastructure::{
        config::Config, database::create_connection, logging::logging_init, state::State,
    },
    preserntation::{grps::grps_init, http::router_init},
};
use std::sync::Arc;
pub(crate) mod application;
pub(crate) mod data;
pub(crate) mod domain;
pub(crate) mod infrastructure;
pub(crate) mod preserntation;
pub(crate) mod utils;

/// Флаг, указывающий на режим разработки
/// (можно подтягивать из конфига, но для простоты оставим константой)
const IS_DEV: bool = true;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    logging_init();

    let config = Config::from_env()?;
    let database = {
        if !IS_DEV {
            let connection = create_connection(&config).await?;
            infrastructure::migrations::run_migrations(&connection).await?;
            Arc::new(Database::Postgres(connection))
        } else {
            Arc::new(Database::Memory(Arc::new(State::new())))
        }
    };
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port_api))
        .await
        .expect("Failed to bind to address");
    let http_router = router_init(&config, database.clone())?;
    let http_server = async { axum::serve(listener, http_router).await };

    let grps_addr = format!("{}:{}", config.host, config.port_grps).parse()?;
    let grps_router = grps_init(&config, database.clone())?;
    let grps_server = async { grps_router.serve(grps_addr).await };
    let (grps, server) = tokio::join!(grps_server, http_server);
    grps.unwrap();
    server.unwrap();

    Ok(())
}
