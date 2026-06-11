use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::Value;
use crate::AppState;
use crate::services::ncco;

#[derive(Deserialize, Default)]
pub struct WebhookBody {
    #[serde(default)] pub dtmf: Option<DtmfPayload>,
    #[serde(default)] pub speech: Option<SpeechPayload>,
}
#[derive(Deserialize)]
pub struct DtmfPayload { #[serde(default)] pub digits: Option<String> }
#[derive(Deserialize)]
pub struct SpeechPayload { #[serde(default)] pub results: Vec<SpeechResult> }
#[derive(Deserialize)]
pub struct SpeechResult { #[serde(default)] pub text: Option<String> }

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/webhooks")
        .route("/answer", web::post().to(answer))
        .route("/answer", web::get().to(answer))
        .route("/input", web::post().to(input))
        .route("/event", web::post().to(event))
        .route("/recording", web::post().to(recording))
        .route("/monitoring-event", web::post().to(ok_200))
        .route("/recording-event", web::post().to(ok_200))
        .route("/broadcast-event", web::post().to(ok_200))
        .route("/composer-event", web::post().to(ok_200))
        .route("/captions-event", web::post().to(ok_200))
        .route("/sip-monitoring-event", web::post().to(ok_200))
    );
}

async fn answer(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let base = data.voice.base_url();
    let ncco = vec![
        ncco::talk("Welcome to Techub Comms. Press 1 for sales, 2 for support, or 3 for billing.", false),
        ncco::input(vec![format!("{}/webhooks/input", base)], 1),
    ];
    HttpResponse::Ok().json(ncco)
}

async fn input(body: web::Json<WebhookBody>, data: web::Data<AppState>) -> HttpResponse {
    let base = data.voice.base_url();
    if let Some(ref dtmf) = body.dtmf {
        if let Some(ref digits) = dtmf.digits {
            let ncco = match digits.as_str() {
                "1" => vec![ncco::talk("Connecting you to sales. Please hold.", true), ncco::connect("15551111111", base, data.voice.vonage_number())],
                "2" => vec![ncco::talk("Connecting you to support. Please hold.", true), ncco::connect("15552222222", base, data.voice.vonage_number())],
                "3" => vec![ncco::talk("Please describe your billing issue after the tone.", true), ncco::record(base)],
                _ => vec![ncco::talk("Invalid option. Goodbye.", false)],
            };
            return HttpResponse::Ok().json(ncco);
        }
    }
    if let Some(ref speech) = body.speech {
        if let Some(ref result) = speech.results.first() {
            if let Some(ref text) = result.text {
                return HttpResponse::Ok().json(vec![ncco::talk(&format!("You said: {}. Let me help you with that.", text), true)]);
            }
        }
    }
    HttpResponse::Ok().json(vec![ncco::talk("We did not receive your input. Goodbye.", false)])
}

async fn event(_req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    log::info!("[Voice Event] {:?}", body);
    HttpResponse::Ok().body("OK")
}

async fn recording(_req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    log::info!("[Recording] {:?}", body);
    HttpResponse::Ok().body("OK")
}

async fn ok_200(_req: HttpRequest, _body: web::Json<Value>) -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
