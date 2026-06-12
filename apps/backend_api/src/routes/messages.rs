use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::security::validation;

#[derive(Deserialize)]
pub struct TextMsgReq {
    pub from: String,
    pub to: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ImageMsgReq {
    pub from: String,
    pub to: String,
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct GroupReq {
    pub owner: String,
    pub group_name: String,
    #[serde(default)] pub members: Vec<String>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/message")
        .route("/send", web::post().to(send_text))
        .route("/send-image", web::post().to(send_image))
        .route("/group", web::post().to(create_group))
    );
}

async fn send_text(body: web::Json<TextMsgReq>, data: web::Data<AppState>) -> HttpResponse {
    let content = match validation::sanitize(&body.content) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };

    match data.message.send_text_message(&body.from, &body.to, &content).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn send_image(body: web::Json<ImageMsgReq>, data: web::Data<AppState>) -> HttpResponse {
    match data.message.send_image_message(&body.from, &body.to, &body.image_url).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn create_group(body: web::Json<GroupReq>, data: web::Data<AppState>) -> HttpResponse {
    match data.message.create_group(&body.owner, &body.group_name, &body.members).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
