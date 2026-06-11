use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::security::validation;

#[derive(Deserialize)]
pub struct MsgReq {
    pub to: String,
    #[serde(default)] pub from: String,
    #[serde(default)] pub text: String,
    #[serde(default)] pub media_url: Vec<String>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/message")
        .route("/send", web::post().to(send_sms))
        .route("/send-mms", web::post().to(send_mms))
        .route("/send-whatsapp", web::post().to(send_whatsapp))
    );
}

async fn send_sms(body: web::Json<MsgReq>, data: web::Data<AppState>) -> HttpResponse {
    if let Err(e) = validation::phone(&body.to) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    let text = match validation::sanitize(&body.text) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };
    let from = if body.from.is_empty() { data.message.vonage_number() } else { &body.from };
    match data.message.send_sms(&body.to, from, &text).await {
        Ok(r) => {
            let mid = r["messages"][0]["message-id"].as_str().unwrap_or("unknown");
            HttpResponse::Ok().json(json!({"messageId": mid, "status": "sent"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn send_mms(body: web::Json<MsgReq>, data: web::Data<AppState>) -> HttpResponse {
    if let Err(e) = validation::phone(&body.to) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    let text = match validation::sanitize(&body.text) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };
    let from = if body.from.is_empty() { data.message.vonage_number() } else { &body.from };
    match data.message.send_mms(&body.to, from, &text, &body.media_url).await {
        Ok(r) => {
            let mid = r["messages"][0]["message-id"].as_str().unwrap_or("unknown");
            HttpResponse::Ok().json(json!({"messageId": mid, "status": "sent"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn send_whatsapp(body: web::Json<MsgReq>, data: web::Data<AppState>) -> HttpResponse {
    if let Err(e) = validation::phone(&body.to) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    let text = match validation::sanitize(&body.text) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };
    let from = if body.from.is_empty() { data.message.vonage_number() } else { &body.from };
    match data.message.send_whatsapp(&body.to, from, &text).await {
        Ok(r) => {
            let mid = r["messages"][0]["message-id"].as_str().unwrap_or("unknown");
            HttpResponse::Ok().json(json!({"messageId": mid, "status": "sent"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
