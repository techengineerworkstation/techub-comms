use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::security::validation;

#[derive(Deserialize)]
pub struct OutboundCallReq {
    pub to: String,
    #[serde(default)] pub from: Option<String>,
    pub channel: String,
    #[serde(default)] pub uid: Option<String>,
    #[serde(default)] pub region: Option<String>,
    #[serde(default)] pub prompt: Option<String>,
}

#[derive(Deserialize)]
pub struct InboundReq {
    pub channel: String,
    #[serde(default)] pub uid: Option<String>,
    #[serde(default)] pub region: Option<String>,
}

#[derive(Deserialize)]
pub struct CallIdReq {
    pub call_id: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/pstn")
        .route("/outbound", web::post().to(outbound_call))
        .route("/inbound", web::post().to(inbound_pstn))
        .route("/end", web::post().to(end_call))
        .route("/cancel", web::post().to(cancel_call))
        .route("/regions", web::get().to(get_regions))
    );
}

async fn outbound_call(body: web::Json<OutboundCallReq>, data: web::Data<AppState>) -> HttpResponse {
    if let Err(e) = validation::phone(&body.to) {
        return HttpResponse::BadRequest().json(json!({"error": e}));
    }

    let from = body.from.as_deref().unwrap_or("+1800222333");
    let uid = body.uid.as_deref().unwrap_or("0");
    let region = body.region.as_deref().unwrap_or("AREA_CODE_NA");
    let prompt = body.prompt.as_deref().unwrap_or("true");

    match data.pstn.outbound_call(&body.to, from, &body.channel, uid, region, prompt).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn inbound_pstn(body: web::Json<InboundReq>, data: web::Data<AppState>) -> HttpResponse {
    let uid = body.uid.as_deref().unwrap_or("0");
    let region = body.region.as_deref().unwrap_or("AREA_CODE_NA");

    match data.pstn.inbound_pstn(&body.channel, uid, region).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn end_call(body: web::Json<CallIdReq>, data: web::Data<AppState>) -> HttpResponse {
    match data.pstn.end_call(&body.call_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn cancel_call(body: web::Json<CallIdReq>, data: web::Data<AppState>) -> HttpResponse {
    match data.pstn.cancel_call(&body.call_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn get_regions(_data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "regions": [
            {"code": "AREA_CODE_NA", "name": "North America", "prefix": "+1"},
            {"code": "AREA_CODE_EU", "name": "Europe", "prefix": "+44"},
            {"code": "AREA_CODE_AS", "name": "Asia", "prefix": "+86"},
            {"code": "AREA_CODE_JP", "name": "Japan", "prefix": "+81"},
            {"code": "AREA_CODE_IN", "name": "India", "prefix": "+91"},
            {"code": "AREA_CODE_CN", "name": "Mainland China", "prefix": "+86"}
        ]
    }))
}
