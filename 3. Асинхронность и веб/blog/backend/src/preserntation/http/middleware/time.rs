use crate::preserntation::http::consts::RequestId;
use axum::{
    extract::{MatchedPath, Request},
    response::Response,
};
use futures_util::future::BoxFuture;
use std::{
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct TimeLayer;

impl<S> Layer<S> for TimeLayer {
    type Service = TimeMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct TimeMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for TimeMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str)
            .unwrap_or("unknow");
        let Some(id) = request.extensions().get::<RequestId>().cloned() else {
            panic!("request id not found");
        };

        info!(
            "http_request: {} - {} - {}",
            request.method(),
            id.0.to_string(),
            matched_path,
        );
        let future = self.inner.call(request);
        let t = Instant::now();
        Box::pin(async move {
            let response: Response = future.await?;
            let elapsed = t.elapsed();
            info!(
                "http_response: {} - {} - {} ms",
                response.status(),
                id.0.to_string(),
                elapsed.as_micros() / 1000,
            );
            Ok(response)
        })
    }
}
