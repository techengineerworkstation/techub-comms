use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::security::validation;
use crate::services::ncco;

#[derive(Deserialize)]
pub struct CallReq {
    pub to: String,
    #[serde(default)] pub from: Option<String>,
    #[serde(default)] pub text: Option<String>,
    #[serde(default)] pub ivr: Option<String>,
    #[serde(default)] pub conference: Option<String>,
    #[serde(default)] pub connect_to: Option<String>,
}

#[derive(Deserialize)]
pub struct ModifyReq { pub action: String }

#[derive(Deserialize)]
pub struct TalkReq {
    pub text: String,
    #[serde(default = "def_lang")] pub language: String,
    #[serde(default = "def_voice")] pub voice_name: String,
}
fn def_lang() -> String { "en-US".into() }
fn def_voice() -> String { "Amy".into() }

#[derive(Deserialize)]
pub struct DtmfReq { pub digits: String }

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/voice")
        .route("/call", web::post().to(create_call))
        .route("/call/{uuid}", web::put().to(modify_call))
        .route("/talk/{uuid}", web::post().to(play_tts))
        .route("/talk/{uuid}", web::delete().to(stop_tts))
        .route("/dtmf/{uuid}", web::post().to(send_dtmf))
    );
}

async fn create_call(body: web::Json<CallReq>, data: web::Data<AppState>) -> HttpResponse {
    if let Err(e) = validation::phone(&body.to) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    let from = body.from.as_deref().unwrap_or(data.voice.vonage_number());
    let base = data.voice.base_url().to_string();

    let ncco = if let Some(ref conf) = body.conference {
        vec![ncco::conversation(conf)]
    } else if let Some(ref ct) = body.connect_to {
        if let Err(e) = validation::phone(ct) { return HttpResponse::BadRequest().json(json!({"error": e})); }
        vec![ncco::connect(ct, &base, data.voice.vonage_number())]
    } else if let Some(ref prompt) = body.ivr {
        ncco::ivr_menu(prompt, vec![format!("{}/webhooks/input", base)])
    } else {
        let t = body.text.as_deref().unwrap_or("Hello from Techub Comms.");
        vec![ncco::talk(t, true)]
    };

    match data.voice.create_outbound_call(&body.to, from, ncco,
        Some(format!("{}/webhooks/answer", base)),
        Some(format!("{}/webhooks/event", base))).await {
        Ok(r) => HttpResponse::Ok().json(json!({"uuid": r["uuid"], "status": "call-initiated"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn modify_call(path: web::Path<String>, body: web::Json<ModifyReq>, data: web::Data<AppState>) -> HttpResponse {
    let uuid = path.into_inner();
    if let Err(e) = validation::uuid(&uuid) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    let allowed = ["hangup", "mute", "unmute", "earmuff", "unearmuff"];
    if !allowed.contains(&body.action.as_str()) { return HttpResponse::BadRequest().json(json!({"error": "Invalid action"})); }
    match data.voice.modify_call(&uuid, &body.action).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": format!("Call {} successful", body.action)})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn play_tts(path: web::Path<String>, body: web::Json<TalkReq>, data: web::Data<AppState>) -> HttpResponse {
    let uuid = path.into_inner();
    if let Err(e) = validation::uuid(&uuid) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    let text = match validation::sanitize(&body.text) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };
    match data.voice.play_tts(&uuid, &text, &body.language, &body.voice_name).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "TTS started"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn stop_tts(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let uuid = path.into_inner();
    match data.voice.stop_tts(&uuid).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "TTS stopped"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn send_dtmf(path: web::Path<String>, body: web::Json<DtmfReq>, data: web::Data<AppState>) -> HttpResponse {
    let uuid = path.into_inner();
    if let Err(e) = validation::uuid(&uuid) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    if let Err(e) = validation::dtmf(&body.digits) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    match data.voice.send_dtmf(&uuid, &body.digits).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "DTMF sent"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
