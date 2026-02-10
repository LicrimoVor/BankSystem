use crate::{data::Database, infrastructure::config::Config};
use anyhow::Result;
use std::sync::Arc;
use tonic::transport::{Server, server::Router};
mod general;
mod post;
mod user;

pub fn grps_init(config: Arc<Config>, database: Arc<Database>) -> Result<Router> {
    // Ok(Server::builder().add_service(todo!()))
    todo!()
}
