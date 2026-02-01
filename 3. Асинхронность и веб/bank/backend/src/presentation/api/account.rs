use crate::{
    application::{account, user},
    data::Database,
    infrastructure::error::ErrorApi,
    presentation::{dto::account::AccountDto, extractor::user::UserExtractor},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/account/create")]
async fn create_account(
    db: web::Data<Database>,
    user: UserExtractor,
) -> actix_web::Result<impl Responder> {
    let user = user::get_user_by_id(db.clone().into_inner(), user.id)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let account = account::create_account(db.into_inner(), &user, None).await?;
    Ok(HttpResponse::Created().json(serde_json::json!(AccountDto::from(account))))
}

#[get("/account/{id}")]
async fn get_account_by_id(
    db: web::Data<Database>,
    user: UserExtractor,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let account_id = path.into_inner();
    let account = account::get_account_by_id(db.into_inner(), account_id)
        .await
        .ok_or(ErrorApi::NotFound("Account not found".to_string()))?;
    if *account.user_id() != user.id {
        return Ok(HttpResponse::Forbidden().finish());
    }
    Ok(HttpResponse::Ok().json(serde_json::json!(AccountDto::from(account))))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_account).service(get_account_by_id);
}
