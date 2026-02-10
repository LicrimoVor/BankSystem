use crate::preserntation::http::consts::RequestId;
use axum::{extract::Request, response::Response};
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
        let future = self.inner.call(request);
        let t = Instant::now();
        Box::pin(async move {
            let response: Response = future.await?;
            let elapsed = t.elapsed();
            if let Some(id) = response.extensions().get::<RequestId>() {
                info!("{} - {} ms", id.0, elapsed.as_millis());
            };
            Ok(response)
        })
    }
}
