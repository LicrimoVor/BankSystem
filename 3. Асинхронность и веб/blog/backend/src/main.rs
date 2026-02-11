use tokio::net::TcpListener;
use tracing::info;

use crate::{
    data::Database,
    infrastructure::{
        config::Config, database::create_connection, logging::logging_init, state::State,
    },
    preserntation::{grpc::grpc_init, http::http_init},
};
use std::{net::SocketAddr, sync::Arc};
pub(crate) mod application;
pub(crate) mod data;
pub(crate) mod domain;
pub(crate) mod infrastructure;
pub(crate) mod preserntation;
pub(crate) mod utils;

/// Флаг, указывающий на режим разработки
/// (можно подтягивать из конфига, но для простоты оставим константой)
const IS_DEV: bool = false;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    logging_init();

    let config = Arc::new(Config::from_env()?);
    let database = {
        if !IS_DEV {
            let connection = create_connection(&config).await?;
            infrastructure::migrations::run_migrations(&connection).await?;
            Arc::new(Database::Postgres(connection))
        } else {
            Arc::new(Database::Memory(Arc::new(State::new())))
        }
    };
    let http_addr = format!("{}:{}", config.host, config.port_api);
    let http_listener = TcpListener::bind(http_addr.clone()).await?;
    let http_router = http_init(config.clone(), database.clone())?;
    let http_server = async { axum::serve(http_listener, http_router).await };
    info!("http server started on {}", http_addr);

    let grpc_addr: SocketAddr = format!("{}:{}", config.host, config.port_grpc).parse()?;
    let grpc_router = grpc_init(config.clone(), database.clone())?;
    let grpc_server = async { grpc_router.serve(grpc_addr.clone()).await };
    info!("grpc server started on {}", grpc_addr);

    let (grpc, server) = tokio::join!(grpc_server, http_server);
    grpc.unwrap();
    server.unwrap();

    Ok(())
}
