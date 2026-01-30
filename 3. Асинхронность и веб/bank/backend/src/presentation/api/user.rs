use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use super::super::dto::user::{LoginDto, RegisterDto, TokenResponse};
use crate::{
    application::user::{create_user, get_user_by_email, get_user_by_id},
    data::Database,
    infrastructure::{config::Config, error::ErrorApi, security},
    presentation::{consts::AUTH_COOKIE, dto::user::UserDto},
};

#[post("/register")]
async fn register(
    db: web::Data<Database>,
    session: Session,
    cfg: web::Data<Config>,
    body: web::Json<RegisterDto>,
) -> actix_web::Result<impl Responder> {
    if session.get::<Uuid>(AUTH_COOKIE)?.is_some() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    let RegisterDto { email, password } = body.into_inner();
    let user = create_user(db.into_inner(), email, password).await?;

    // ПЕРЕДЕЛАТЬ КУКИ
    session.insert(AUTH_COOKIE, user.id())?;

    Ok(HttpResponse::Created().json(serde_json::json!(UserDto::from(user))))
}

#[post("/login")]
async fn login(
    db: web::Data<Database>,
    session: Session,
    cfg: web::Data<Config>,
    body: web::Json<LoginDto>,
) -> actix_web::Result<impl Responder> {
    let email = body.email.trim().to_lowercase();
    let user = get_user_by_email(db.into_inner(), email)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let ok = security::verify_password(&body.password, &user.password_hash())
        .map_err(|_| actix_web::error::ErrorInternalServerError("verify error"))?;

    if !ok {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let token = security::generate_jwt(&cfg.jwt_secret, *user.id())
        .map_err(|_| actix_web::error::ErrorInternalServerError("jwt error"))?;

    // ПЕРЕДЕЛАТЬ КУКИ
    session.insert(AUTH_COOKIE, user.id())?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
    }))
}

#[get("/me")]
async fn me(db: web::Data<Database>, session: Session) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(AUTH_COOKIE)? else {
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
