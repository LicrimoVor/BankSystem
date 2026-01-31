use crate::{
    application::{transaction, user},
    data::Database,
    infrastructure::error::ErrorApi,
    presentation::{
        consts::REFRESH_COOKIE,
        dto::transaction::{DepositDto, TransactionDto, TransferDto, WithdrawalDto},
    },
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/account/{id}/deposit")]
async fn deposit(
    db: web::Data<Database>,
    session: Session,
    body: web::Json<DepositDto>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(REFRESH_COOKIE)? else {
        return Ok(HttpResponse::Forbidden().finish());
    };
    let account_id = path.into_inner();
    let DepositDto { amount } = body.into_inner();

    let user = user::get_user_by_id(db.clone().into_inner(), user_id)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let transaction = transaction::deposit(db.into_inner(), &user, account_id, amount).await?;
    Ok(HttpResponse::Created().json(serde_json::json!(TransactionDto::from(transaction))))
}

#[post("/account/{id}/withdrawal")]
async fn withdrawal(
    db: web::Data<Database>,
    session: Session,
    body: web::Json<WithdrawalDto>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(REFRESH_COOKIE)? else {
        return Ok(HttpResponse::Forbidden().finish());
    };
    let account_id = path.into_inner();
    let WithdrawalDto { amount } = body.into_inner();

    let user = user::get_user_by_id(db.clone().into_inner(), user_id)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let transaction = transaction::withdraw(db.into_inner(), &user, account_id, amount).await?;
    Ok(HttpResponse::Created().json(serde_json::json!(TransactionDto::from(transaction))))
}

#[post("/account/{id}/transfer")]
async fn transfer(
    db: web::Data<Database>,
    session: Session,
    body: web::Json<TransferDto>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let Some(user_id) = session.get::<Uuid>(REFRESH_COOKIE)? else {
        return Ok(HttpResponse::Forbidden().finish());
    };
    let account_id = path.into_inner();
    let TransferDto {
        amount,
        to_account_id,
    } = body.into_inner();

    let user = user::get_user_by_id(db.clone().into_inner(), user_id)
        .await
        .ok_or(ErrorApi::NotFound("User not found".to_string()))?;

    let transaction =
        transaction::transfer(db.into_inner(), &user, account_id, to_account_id, amount).await?;
    Ok(HttpResponse::Created().json(serde_json::json!(TransactionDto::from(transaction))))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(deposit).service(withdrawal).service(transfer);
}
