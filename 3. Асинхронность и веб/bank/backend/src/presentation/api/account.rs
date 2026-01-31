use crate::{
    application::{account, user},
    data::Database,
    infrastructure::error::ErrorApi,
    presentation::{consts::REFRESH_COOKIE, dto::account::AccountDto},
};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/account/create")]
async fn create_account(
    db: web::Data<Database>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(REFRESH_COOKIE)? else {
        return Ok(HttpResponse::Forbidden().finish());
    };

    let user = user::get_user_by_id(db.clone().into_inner(), user_id)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let account = account::create_account(db.into_inner(), &user, None).await?;
    Ok(HttpResponse::Created().json(serde_json::json!(AccountDto::from(account))))
}

#[get("/account/{id}")]
async fn get_account_by_id(
    db: web::Data<Database>,
    session: Session,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(REFRESH_COOKIE)? else {
        return Ok(HttpResponse::Forbidden().finish());
    };
    let account_id = path.into_inner();
    let account = account::get_account_by_id(db.into_inner(), account_id)
        .await
        .ok_or(ErrorApi::NotFound("Account not found".to_string()))?;
    if *account.user_id() != user_id {
        return Ok(HttpResponse::Forbidden().finish());
    }
    Ok(HttpResponse::Ok().json(serde_json::json!(AccountDto::from(account))))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_account).service(get_account_by_id);
}
