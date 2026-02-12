use super::proto::general::*;
use crate::{
    dto,
    grpc::GrpcState,
    types::{Error, general::GeneralClientTrait},
};
use std::sync::{Arc, Mutex};
use tonic::Request;

pub struct GeneralClient {
    inner: general_service_client::GeneralServiceClient<tonic::transport::Channel>,
    state: Arc<Mutex<GrpcState>>,
}

impl GeneralClient {
    pub(super) fn new(channel: tonic::transport::Channel, state: Arc<Mutex<GrpcState>>) -> Self {
        Self {
            inner: general_service_client::GeneralServiceClient::new(channel),
            state,
        }
    }
}

#[async_trait::async_trait]
impl GeneralClientTrait for GeneralClient {
    async fn health(&mut self) -> Result<String, Error> {
        let HealthResponse { status } = self
            .inner
            .health(Request::new(dto::Empty {}))
            .await?
            .into_inner();
        Ok(status)
    }

    async fn ping(&mut self) -> Result<String, Error> {
        let PingResponse { pong } = self
            .inner
            .ping(Request::new(PingRequest {
                ping: "ping".to_string(),
            }))
            .await?
            .into_inner();
        Ok(pong)
    }
}
