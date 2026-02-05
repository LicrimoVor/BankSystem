use std::future::{ready, Ready};
use std::task::{Context, Poll};

use actix_web::cookie::Cookie;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

use crate::infrastructure::error::ErrorApi;
use crate::presentation::consts::{CSRF_TOKEN_COOKIE, CSRF_TOKEN_HEADER, EXCLUDE_PATHS};

pub struct CsrfMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CsrfMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CsrfService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfService { service }))
    }
}

pub struct CsrfService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CsrfService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (csrf_token, is_created) = match req.cookie(CSRF_TOKEN_COOKIE) {
            Some(cookie) => (cookie.value().to_string(), false),
            None => (Uuid::new_v4().to_string(), true),
        };

        if !(EXCLUDE_PATHS.contains(&req.path()))
            & vec!["POST", "PUT", "DELETE", "PATCH"].contains(&req.method().as_str())
        {
            let Some(csrf_token_header) = req.headers().get(CSRF_TOKEN_HEADER) else {
                return Box::pin(async move {
                    Err(ErrorApi::Forbidden("Missing csrf token header".to_string()).into())
                });
            };

            if csrf_token != csrf_token_header.to_str().unwrap() {
                return Box::pin(async move {
                    Err(ErrorApi::Forbidden("Invalid csrf token".to_string()).into())
                });
            }
        }

        req.extensions_mut().insert(csrf_token.clone());
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            if is_created {
                res.response_mut().add_cookie(
                    &Cookie::build(CSRF_TOKEN_COOKIE, csrf_token)
                        .path("/api")
                        .finish(),
                )?;
            }
            Ok(res)
        })
    }
}
