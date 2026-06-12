use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::security::validation;

#[derive(Deserialize)]
pub struct TokenReq {
    pub channel: String,
    #[serde(default)] pub uid: Option<u32>,
    #[serde(default)] pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct RecordingReq {
    pub channel: String,
    #[serde(default)] pub uid: Option<u32>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/video")
        .route("/token", web::post().to(get_token))
        .route("/channel", web::post().to(create_channel))
        .route("/recording/start", web::post().to(start_recording))
        .route("/recording/stop", web::post().to(stop_recording))
        .route("/config", web::get().to(get_config))
    );
}

async fn get_config(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "appId": data.video.app_id(),
        "status": "ok"
    }))
}

async fn get_token(body: web::Json<TokenReq>, data: web::Data<AppState>) -> HttpResponse {
    let channel = body.channel.clone();
    if let Err(e) = validation::room(&channel) {
        return HttpResponse::BadRequest().json(json!({"error": e}));
    }

    let uid = body.uid.unwrap_or(0);
    let role = body.role.as_deref().unwrap_or("publisher");

    match data.video.generate_token(&channel, uid, role) {
        Ok((token, app_id)) => HttpResponse::Ok().json(json!({
            "token": token,
            "appId": app_id,
            "channel": channel,
            "uid": uid
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn create_channel(body: web::Json<TokenReq>, data: web::Data<AppState>) -> HttpResponse {
    let channel = body.channel.clone();
    if let Err(e) = validation::room(&channel) {
        return HttpResponse::BadRequest().json(json!({"error": e}));
    }

    match data.video.create_channel(&channel).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn start_recording(body: web::Json<RecordingReq>, data: web::Data<AppState>) -> HttpResponse {
    let uid = body.uid.unwrap_or(0);
    match data.video.start_recording(&body.channel, uid).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn stop_recording(body: web::Json<serde_json::Value>, data: web::Data<AppState>) -> HttpResponse {
    let resource_id = body.get("resourceId").and_then(|v| v.as_str()).unwrap_or("");
    let sid = body.get("sid").and_then(|v| v.as_str()).unwrap_or("");
    match data.video.stop_recording(resource_id, sid).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
