mod account;
mod general;
mod user;

use general::configure as general_configure;
use user::configure as user_configure;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    general_configure(cfg);
    user_configure(cfg);
}
