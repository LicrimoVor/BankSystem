use std::sync::Arc;
use uuid::Uuid;

use crate::{
    data::Database,
    domain::{token::RefreshToken, user::User},
    infrastructure::{config::Config, error::ErrorApi, security},
    presentation::extractor::refresh,
};

pub async fn create_user(
    db: Arc<Database>,
    cfg: Arc<Config>,
    email: String,
    password: String,
) -> Result<(User, String, String), ErrorApi> {
    let email = email.trim().to_lowercase();
    let mut user_repo = db.clone().get_user_repo();
    let mut token_repo = db.get_refresh_token_repo();
    let user = user_repo.create(email, password).await?;

    let jwt_token = security::generate_jwt(&cfg.jwt_secret, *user.id())
        .map_err(|_| ErrorApi::Inner("jwt error".to_string()))?;
    let refresh_token = security::generate_refresh_token();
    let _ = token_repo.create(refresh_token.clone(), *user.id()).await?;

    Ok((user, refresh_token, jwt_token))
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
) -> Result<(User, String, String), ErrorApi> {
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
    let refresh_token = security::generate_refresh_token();
    let _ = token_repo.create(refresh_token.clone(), *user.id()).await?;

    Ok((user, refresh_token, jwt_token))
}

pub async fn logout_user(db: Arc<Database>, refresh_token: String) -> Result<(), ErrorApi> {
    let mut repo = db.get_refresh_token_repo();
    repo.delete(refresh_token).await?;
    Ok(())
}

pub async fn refresh_jwt_token(
    db: Arc<Database>,
    cfg: Arc<Config>,
    refresh_token: String,
) -> Result<String, ErrorApi> {
    let repo = db.get_refresh_token_repo();
    let token = repo.get(refresh_token).await?;

    let jwt_token = security::generate_jwt(&cfg.jwt_secret, *token.user_id())
        .map_err(|_| ErrorApi::Inner("jwt error".to_string()))?;
    Ok(jwt_token)
}
