use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::AppState;
use crate::security::validation;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/video")
        .route("/session/{room}", web::get().to(get_session))
        .route("/session/{room}/startArchive", web::post().to(start_archive))
        .route("/session/{room}/{archive_id}/stopArchive", web::post().to(stop_archive))
        .route("/session/{room}/archives", web::get().to(list_archives))
        .route("/session/{room}/enableCaptions", web::post().to(enable_captions))
        .route("/session/{room}/{captions_id}/disableCaptions", web::post().to(disable_captions))
    );
}

async fn get_session(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let room = path.into_inner();
    if let Err(e) = validation::room(&room) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    match data.video.get_or_create_session(&room).await {
        Ok(sid) => match data.video.generate_token(&sid) {
            Ok((tok, key)) => HttpResponse::Ok().json(json!({"sessionId": sid, "token": tok, "apiKey": key})),
            Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn start_archive(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let room = path.into_inner();
    if let Err(e) = validation::room(&room) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    match data.video.get_or_create_session(&room).await {
        Ok(sid) => match data.video.start_archive(&room, &sid).await {
            Ok(a) => HttpResponse::Ok().json(json!({"archiveId": a["id"], "status": 200})),
            Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn stop_archive(path: web::Path<(String, String)>, data: web::Data<AppState>) -> HttpResponse {
    let (_room, aid) = path.into_inner();
    if let Err(e) = validation::uuid(&aid) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    match data.video.stop_archive(&aid).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Archive stopped", "status": 200})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn list_archives(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let room = path.into_inner();
    if let Err(e) = validation::room(&room) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    match data.video.get_or_create_session(&room).await {
        Ok(sid) => match data.video.list_archives(&sid).await {
            Ok(items) => HttpResponse::Ok().json(json!({"archives": items, "status": 200})),
            Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn enable_captions(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let room = path.into_inner();
    if let Err(e) = validation::room(&room) { return HttpResponse::BadRequest().json(json!({"error": e})); }
    match data.video.get_or_create_session(&room).await {
        Ok(sid) => match data.video.enable_captions(&sid).await {
            Ok(c) => HttpResponse::Ok().json(json!({"captionsId": c["captionsId"], "status": 200})),
            Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}

async fn disable_captions(path: web::Path<(String, String)>, data: web::Data<AppState>) -> HttpResponse {
    let (_room, cid) = path.into_inner();
    match data.video.disable_captions(&cid).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Captions stopped", "status": 200})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e})),
    }
}
