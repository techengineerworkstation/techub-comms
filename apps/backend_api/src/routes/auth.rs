use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use crate::auth;

#[derive(Deserialize)]
pub struct RegisterReq {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/auth")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
        .route("/me", web::get().to(me))
    );
}

async fn register(body: web::Json<RegisterReq>, pool: web::Data<PgPool>) -> HttpResponse {
    let req = auth::RegisterRequest {
        email: body.email.clone(),
        password: body.password.clone(),
        display_name: body.display_name.clone(),
    };

    match auth::register_user(pool.get_ref(), &req).await {
        Ok(token) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "token": token,
            "message": "Registration successful"
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

async fn login(body: web::Json<LoginReq>, pool: web::Data<PgPool>) -> HttpResponse {
    let req = auth::LoginRequest {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    match auth::login_user(pool.get_ref(), &req).await {
        Ok(token) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "token": token,
            "message": "Login successful"
        })),
        Err(e) => HttpResponse::Unauthorized().json(json!({"error": e})),
    }
}

async fn logout(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Some(token) = extract_token(&req) {
        let _ = auth::logout(pool.get_ref(), &token).await;
    }
    HttpResponse::Ok().json(json!({"status": "ok", "message": "Logged out"}))
}

async fn me(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let token = match extract_token(&req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().json(json!({"error": "No token"})),
    };

    match auth::get_user_from_token(pool.get_ref(), &token).await {
        Ok((user_id, email, name)) => HttpResponse::Ok().json(json!({
            "user_id": user_id,
            "email": email,
            "display_name": name
        })),
        Err(e) => HttpResponse::Unauthorized().json(json!({"error": e})),
    }
}

fn extract_token(req: &HttpRequest) -> Option<String> {
    // Authorization header
    if let Some(auth) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }
    // Cookie
    if let Some(cookie) = req.cookie("techub_token") {
        return Some(cookie.value().to_string());
    }
    None
}
