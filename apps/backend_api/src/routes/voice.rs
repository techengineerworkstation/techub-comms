use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::security::validation;

#[derive(Deserialize)]
pub struct CallReq {
    pub to: String,
    #[serde(default)] pub from: Option<String>,
    #[serde(default)] pub channel: Option<String>,
    #[serde(default)] pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct SignalReq {
    pub from: String,
    pub to: String,
    #[serde(default)] pub signal_type: Option<String>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/voice")
        .route("/call", web::post().to(initiate_call))
        .route("/signal", web::post().to(send_signal))
        .route("/channel", web::post().to(create_channel))
    );
}

async fn initiate_call(body: web::Json<CallReq>, data: web::Data<AppState>) -> HttpResponse {
    if let Err(e) = validation::phone(&body.to) {
        return HttpResponse::BadRequest().json(json!({"error": e}));
    }

    let channel = body.channel.clone().unwrap_or_else(|| {
        format!("call-{}", chrono::Utc::now().timestamp_millis())
    });

    match data.voice.create_call_room(&channel).await {
        Ok(result) => HttpResponse::Ok().json(json!({
            "channel": channel,
            "appId": data.voice.agora_app_id(),
            "status": "call-initiated",
            "details": result
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn send_signal(body: web::Json<SignalReq>, data: web::Data<AppState>) -> HttpResponse {
    let signal_type = body.signal_type.as_deref().unwrap_or("call");
    match data.voice.send_call_signal(&body.from, &body.to, signal_type).await {
        Ok(result) => HttpResponse::Ok().json(json!({"status": "signal-sent", "details": result})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn create_channel(body: web::Json<CallReq>, data: web::Data<AppState>) -> HttpResponse {
    let channel = body.channel.clone().unwrap_or_else(|| {
        format!("voice-{}", chrono::Utc::now().timestamp_millis())
    });

    match data.voice.create_call_room(&channel).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
