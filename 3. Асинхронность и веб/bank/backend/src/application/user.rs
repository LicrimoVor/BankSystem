use std::sync::Arc;
use uuid::Uuid;

use crate::{
    data::Database,
    domain::{token::RefreshToken, user::User},
    infrastructure::{
        config::Config,
        error::ErrorApi,
        security::{self, hash_password},
    },
};

pub async fn create_user(
    db: Arc<Database>,
    cfg: Arc<Config>,
    email: String,
    password: String,
) -> Result<(User, RefreshToken, String), ErrorApi> {
    let email = email.trim().to_lowercase();
    let mut user_repo = db.clone().get_user_repo();
    let mut token_repo = db.get_refresh_token_repo();
    let user = user_repo.create(email, password).await?;

    let jwt_token = security::generate_jwt(&cfg.jwt_secret, *user.id())
        .map_err(|_| ErrorApi::Inner("jwt error".to_string()))?;
    let token = token_repo
        .create(security::generate_refresh_token(), *user.id())
        .await?;

    Ok((user, token, jwt_token))
}

pub async fn delete_user(db: Arc<Database>, user: &User) -> Result<(), ErrorApi> {
    let mut repo = db.get_user_repo();
    repo.delete(user).await
}

pub async fn get_user_by_id(db: Arc<Database>, id: Uuid) -> Option<User> {
    let repo = db.get_user_repo();
    repo.get_by_id(id).await
}

pub async fn login_user(
    db: Arc<Database>,
    cfg: Arc<Config>,
    email: String,
    password: String,
) -> Result<(User, RefreshToken, String), ErrorApi> {
    let email = email.trim().to_lowercase();
    let user_repo = db.clone().get_user_repo();
    let mut token_repo = db.get_refresh_token_repo();

    let user = user_repo
        .get_by_email(email)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let ok = security::verify_password(&password, &user.password_hash())
        .map_err(|_| ErrorApi::Inner("verify error".to_string()))?;

    if !ok {
        return Err(ErrorApi::Validation("Invalid password".to_string()));
    }

    let jwt_token = security::generate_jwt(&cfg.jwt_secret, *user.id())
        .map_err(|_| ErrorApi::Inner("jwt error".to_string()))?;
    let token = token_repo
        .create(security::generate_refresh_token(), *user.id())
        .await?;

    Ok((user, token, jwt_token))
}

pub async fn logout_user(db: Arc<Database>, refresh_token: String) -> Result<(), ErrorApi> {
    let mut repo = db.get_refresh_token_repo();
    let token_refresh_hash =
        hash_password(&refresh_token).map_err(|_| ErrorApi::Inner("hash error".to_string()))?;
    repo.delete(token_refresh_hash).await?;
    Ok(())
}

pub async fn refresh_jwt_token(
    db: Arc<Database>,
    cfg: Arc<Config>,
    refresh_token: String,
) -> Result<String, ErrorApi> {
    let repo = db.get_refresh_token_repo();
    let token_refresh_hash =
        hash_password(&refresh_token).map_err(|_| ErrorApi::Inner("hash error".to_string()))?;
    let token = repo.get(token_refresh_hash).await?;

    let jwt_token = security::generate_jwt(&cfg.jwt_secret, *token.user_id())
        .map_err(|_| ErrorApi::Inner("jwt error".to_string()))?;
    Ok(jwt_token)
}
