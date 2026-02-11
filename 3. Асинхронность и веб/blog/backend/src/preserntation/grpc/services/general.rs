use super::super::{dto, general_service::*};
use crate::{data::Database, infrastructure::config::Config, preserntation::grpc::ResultService};
use std::sync::Arc;
use tonic::{Request, Response};

pub struct GeneralGRPCService {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
}

#[tonic::async_trait]
impl general_service_server::GeneralService for GeneralGRPCService {
    async fn health(&self, _: Request<dto::Empty>) -> ResultService<HealthResponse> {
        Ok(Response::new(HealthResponse {
            status: "OK".to_string(),
        }))
    }
    async fn ping(&self, _: Request<PingRequest>) -> ResultService<PingResponse> {
        Ok(Response::new(PingResponse {
            pong: "pong".to_string(),
        }))
    }
}

pub fn init(
    database: Arc<Database>,
    config: Arc<Config>,
) -> general_service_server::GeneralServiceServer<GeneralGRPCService> {
    general_service_server::GeneralServiceServer::new(GeneralGRPCService { database, config })
}
