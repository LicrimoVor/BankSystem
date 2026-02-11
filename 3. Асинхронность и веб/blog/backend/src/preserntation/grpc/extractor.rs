use crate::{
    domain::auth::JwtToken,
    infrastructure::{config::Config, errors::ErrorBlog},
};
use std::sync::Arc;
use tonic::{Request, Status};
use uuid::Uuid;

pub fn extract_user_id<T>(config: &Arc<Config>, request: &Request<T>) -> Result<Uuid, Status> {
    let Some(token) = request.extensions().get::<JwtToken>() else {
        return Err(ErrorBlog::Unauthorized("Jwt token not found".to_string()).into());
    };
    let a = token.verify(&config.jwt_secret)?;
    Ok(a.sub)
}
