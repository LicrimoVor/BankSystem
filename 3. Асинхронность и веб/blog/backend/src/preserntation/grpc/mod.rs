pub(self) mod consts;
pub(self) mod extractor;
mod interceptor;
mod services;
pub(self) mod types;
use crate::{
    data::Database,
    infrastructure::{config::Config, errors::ErrorBlog},
    preserntation::grpc::interceptor::{
        jwt::jwt_interceptor, req_id::req_id_interceptor, time::TimeLayer,
    },
};
use anyhow::Result;
use std::sync::Arc;
use tonic::{service::InterceptorLayer, transport::Server};
use types::*;

pub fn grpc_init(config: Arc<Config>, database: Arc<Database>) -> Result<RouterType, ErrorBlog> {
    let layer = tower::ServiceBuilder::new()
        .layer(InterceptorLayer::new(req_id_interceptor as InterceptorFn))
        .layer(TimeLayer::default())
        .layer(InterceptorLayer::new(jwt_interceptor as InterceptorFn))
        .into_inner();

    Ok(Server::builder()
        .layer(layer)
        .add_service(services::general::init(database.clone(), config.clone()))
        .add_service(services::auth::init(database.clone(), config.clone()))
        .add_service(services::user::init(database.clone(), config.clone()))
        .add_service(services::post::init(database.clone(), config.clone())))
}
