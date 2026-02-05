use crate::{application, infrastructure::config::Config};
use actix_web::{
    get,
    web::{Data, ServiceConfig},
};
use reqwest::Client;

#[get("/course")]
async fn get_course(
    db: Data<crate::data::Database>,
    client: Data<Client>,
    config: Data<Config>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let course =
        application::course::get_course(db.into_inner(), client.into_inner(), config.into_inner())
            .await?;
    Ok(actix_web::HttpResponse::Ok().json(course))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(get_course);
}
