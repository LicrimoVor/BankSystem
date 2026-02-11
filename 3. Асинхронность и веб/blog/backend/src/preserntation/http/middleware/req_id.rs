use crate::preserntation::http::consts::HEADER_X_ID_REQUEST;

use super::super::consts::RequestId;
use axum::{extract::Request, http::HeaderValue, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for RequestIdMiddleware<S>
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

    fn call(&mut self, mut request: Request) -> Self::Future {
        let id = RequestId(Uuid::new_v4());
        let header = HeaderValue::from_str(&id.0.to_string()).unwrap();

        request.extensions_mut().insert(id.clone());
        let future = self.inner.call(request);
        Box::pin(async move {
            let mut response: Response = future.await?;
            response.headers_mut().insert(HEADER_X_ID_REQUEST, header);
            Ok(response)
        })
    }
}
