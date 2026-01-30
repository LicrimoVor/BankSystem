mod account;
mod general;
pub mod transaction;
mod user;

use account::configure as account_configure;
use actix_web::web;
use general::configure as general_configure;
use transaction::configure as transaction_configure;
use user::configure as user_configure;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(general_configure)
            .configure(user_configure)
            .configure(account_configure)
            .configure(transaction_configure),
    );
}
