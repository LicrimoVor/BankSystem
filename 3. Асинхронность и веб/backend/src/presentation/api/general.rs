use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status":"ok"}))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health);
}
