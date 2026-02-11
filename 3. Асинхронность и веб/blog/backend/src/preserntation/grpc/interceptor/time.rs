use axum::http;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tonic::{Request, Response, Status};
use tower::{Layer, Service};
use tracing::info;

use crate::preserntation::grpc::RequestId;

#[derive(Clone, Default)]
pub struct TimeLayer;

impl<S> Layer<S> for TimeLayer {
    type Service = TimeInterceptor<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeInterceptor { inner }
    }
}

#[derive(Clone)]
pub struct TimeInterceptor<S> {
    inner: S,
}
type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for TimeInterceptor<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<ReqBody>) -> Self::Future {
        let method = request.method().as_str();
        let path = request.uri().path().to_string();

        let Some(id) = request.extensions().get::<RequestId>().cloned() else {
            panic!("request id not found");
        };

        info!("grpc_request: {} - {} - {}", method, id.0.to_string(), path);
        let future = self.inner.call(request);
        let t = Instant::now();
        Box::pin(async move {
            let response = future.await?;
            let status = response.status();
            let elapsed = t.elapsed();

            info!(
                "grpc_response: {} - {} - {} ms",
                status,
                id.0.to_string(),
                elapsed.as_micros() / 1000,
            );
            Ok(response)
        })
    }
}
