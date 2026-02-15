use crate::{
    preserntation::http::consts::{
        COOKIE_CSRF_TOKEN, EXCLUDE_CSRF_PATHS, HEADER_CSRF_TOKEN, METHODS_CSRF, RequestId,
    },
    utils::cookie::{extract_cookie, set_cookie},
};
use axum::{extract::Request, response::Response};
use cookie::{Cookie, SameSite};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct CsrfLayer;

impl<S> Layer<S> for CsrfLayer {
    type Service = CsrfMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CsrfMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct CsrfMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for CsrfMiddleware<S>
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
        // Исключения от CSRF
        if !METHODS_CSRF.contains(request.method())
            || EXCLUDE_CSRF_PATHS.contains(&request.uri().path())
        {
            let is_cookie = extract_cookie(request.headers(), COOKIE_CSRF_TOKEN).is_some();
            let future = self.inner.call(request);
            return Box::pin(async move {
                let mut response = future.await?;
                if !is_cookie {
                    let cookie = Cookie::build((COOKIE_CSRF_TOKEN, Uuid::new_v4().to_string()))
                        .path("/")
                        .http_only(false)
                        .same_site(SameSite::Lax)
                        // .secure(true)
                        .build();
                    set_cookie(&mut response, cookie);
                }
                Ok(response)
            });
        }

        let id = request
            .extensions()
            .get::<RequestId>()
            .map(|id| id.0.to_string())
            .unwrap_or("unknown".to_string());

        let wrong_answer = Box::pin(async move {
            info!("Csrf forbidden: {}", id);

            let response = Response::builder()
                .status(403)
                .body("Csrf forbidden".into())
                .unwrap();
            Ok(response)
        });

        // Проверка csrf
        let Some(cookie) = extract_cookie(request.headers(), COOKIE_CSRF_TOKEN) else {
            return wrong_answer;
        };
        let Some(header) = request.headers().get(HEADER_CSRF_TOKEN) else {
            return wrong_answer;
        };
        let Ok(header) = header.to_str() else {
            return wrong_answer;
        };
        if header != cookie.as_str() {
            return wrong_answer;
        }

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}
