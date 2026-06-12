mod routes;
mod security;
mod services;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, middleware::Logger};
use actix_files as fs;
use shared_core::AppConfig;
use services::{VideoService, VoiceService, MessageService};
use security::rate_limit::RateLimiter;
use security::headers::SecurityHeaders;

pub struct AppState {
    pub config: AppConfig,
    pub video: VideoService,
    pub voice: VoiceService,
    pub message: MessageService,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn spa_fallback(_req: HttpRequest) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let port = config.server_port;
    let frontend_url = config.frontend_url.clone();

    let video = VideoService::new(&config);
    let voice = VoiceService::new(&config);
    let message = MessageService::new(&config);

    let state = web::Data::new(AppState { config, video, voice, message });

    log::info!("Techub Comms Server starting on port {}", port);
    log::info!("Serving API routes and static frontend from ./static/");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("https://thbtechub.sbs")
            .allowed_origin("https://api.thbtechub.sbs")
            .allowed_methods(["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers([
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(SecurityHeaders)
            .wrap(RateLimiter::new(60, 100))
            .app_data(state.clone())
            .app_data(
                web::JsonConfig::default()
                    .limit(10240)
                    .error_handler(|err, _| {
                        actix_web::error::InternalError::from_response(
                            err,
                            HttpResponse::BadRequest().json(serde_json::json!({
                                "error": "Invalid request body"
                            })),
                        )
                        .into()
                    }),
            )
            // API routes
            .route("/health", web::get().to(health))
            .configure(routes::video::configure)
            .configure(routes::voice::configure)
            .configure(routes::messages::configure)
            .configure(routes::webhooks::configure)
            // Static files - serve actual files with correct MIME types
            .service(
                fs::Files::new("/", "./static")
                    .prefer_utf8(true)
                    .use_last_modified(true)
            )
            // SPA fallback for client-side routing (only for non-file routes)
            .default_service(web::to(spa_fallback))
    })
    .bind(("0.0.0.0", port))?
    .workers(num_cpus::get().max(2))
    .run()
    .await
}
