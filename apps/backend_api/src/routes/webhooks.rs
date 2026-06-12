use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;
use crate::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/webhooks")
        .route("/event", web::post().to(event))
        .route("/recording", web::post().to(recording))
    );
}

async fn event(_req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    log::info!("[Agora Event] {:?}", body);
    HttpResponse::Ok().body("OK")
}

async fn recording(_req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    log::info!("[Agora Recording] {:?}", body);
    HttpResponse::Ok().body("OK")
}
