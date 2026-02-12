use super::proto::general::*;
use crate::{dto, grpc::GrpcState};
use std::sync::{Arc, Mutex};
use tonic::{Request, Status};

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

    pub async fn health(&mut self) -> Result<String, Status> {
        let HealthResponse { status } = self
            .inner
            .health(Request::new(dto::Empty {}))
            .await?
            .into_inner();
        Ok(status)
    }

    pub async fn ping(&mut self, ping: String) -> Result<String, Status> {
        let PingResponse { pong } = self
            .inner
            .ping(Request::new(PingRequest { ping }))
            .await?
            .into_inner();
        Ok(pong)
    }
}
