use actix_session::Session;
use actix_web::{
    cookie::{Cookie, SameSite},
    get, post, web, HttpResponse, Responder,
};
use uuid::Uuid;

use super::super::dto::user::{LoginDto, RegisterDto, TokenResponse};
use crate::{
    application::user::{create_user, get_user_by_id, login_user, refresh_jwt_token},
    data::Database,
    infrastructure::{config::Config, error::ErrorApi, security},
    presentation::{
        consts::REFRESH_COOKIE,
        dto::user::{UserDto, UserLoginDto},
        extractor::refresh::RefreshTokenExtractor,
    },
};

#[post("/auth/register")]
async fn register(
    db: web::Data<Database>,
    session: Session,
    cfg: web::Data<Config>,
    body: web::Json<RegisterDto>,
) -> actix_web::Result<impl Responder> {
    if session.get::<Uuid>(REFRESH_COOKIE)?.is_some() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    let RegisterDto { email, password } = body.into_inner();
    let (user, token, jwt_token) =
        create_user(db.into_inner(), cfg.into_inner(), email, password).await?;

    let cookie = Cookie::build(REFRESH_COOKIE, token.refresh_token_hash())
        .path("/auth/refresh")
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::days(31 * 6))
        .finish();

    Ok(HttpResponse::Created().cookie(cookie).json(UserLoginDto {
        id: user.id().clone(),
        email: user.email().clone(),
        access_token: jwt_token,
        refresh_expires_at: token.expires_at().to_string(),
    }))
}

#[post("/auth/login")]
async fn login(
    db: web::Data<Database>,
    session: Session,
    cfg: web::Data<Config>,
    body: web::Json<LoginDto>,
) -> actix_web::Result<impl Responder> {
    if session.get::<Uuid>(REFRESH_COOKIE)?.is_some() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let (user, token, jwt_token) = login_user(
        db.into_inner(),
        cfg.into_inner(),
        body.email.clone(),
        body.password.clone(),
    )
    .await?;

    let cookie = Cookie::build(REFRESH_COOKIE, token.refresh_token_hash())
        .path("/auth/refresh")
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::days(31 * 6))
        .finish();

    Ok(HttpResponse::Created().cookie(cookie).json(UserLoginDto {
        id: user.id().clone(),
        email: user.email().clone(),
        access_token: jwt_token,
        refresh_expires_at: token.expires_at().to_string(),
    }))
}

async fn refresh_token(
    db: web::Data<Database>,
    cfg: web::Data<Config>,
    refresh_token: RefreshTokenExtractor,
) -> actix_web::Result<impl Responder> {
    let jwt_token = refresh_jwt_token(db.into_inner(), cfg.into_inner(), refresh_token.0).await?;
    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: jwt_token,
    }))
}

#[get("/me")]
async fn me(db: web::Data<Database>, session: Session) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(REFRESH_COOKIE)? else {
        return Ok(HttpResponse::Forbidden().finish());
    };

    let user = get_user_by_id(db.clone().into_inner(), user_id)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!(UserDto::from(user))))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(register).service(login).service(me);
}
