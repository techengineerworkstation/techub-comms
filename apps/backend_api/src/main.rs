mod routes;
mod security;
mod services;
mod db;
mod auth;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_files as fs;
use shared_core::AppConfig;
use services::{VideoService, VoiceService, MessageService};
use security::rate_limit::RateLimiter;
use security::headers::SecurityHeaders;
use sqlx::PgPool;

pub struct AppState {
    pub config: AppConfig,
    pub video: VideoService,
    pub voice: VoiceService,
    pub message: MessageService,
    pub db: PgPool,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let port = config.server_port;
    let frontend_url = config.frontend_url.clone();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/techub_comms".to_string());

    let db_pool = match db::create_pool(&database_url).await {
        Ok(pool) => {
            log::info!("Connected to PostgreSQL");
            if let Err(e) = db::run_migrations(&pool).await {
                log::error!("Migration error: {}", e);
            }
            pool
        }
        Err(e) => {
            log::warn!("PostgreSQL connection failed: {}. Running without database.", e);
            // Create a dummy pool - app can still work for static files
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .connect(&database_url)
                .await
                .expect("Failed to create fallback DB pool")
        }
    };

    let video = VideoService::new(&config);
    let voice = VoiceService::new(&config);
    let message = MessageService::new(&config);

    let state = web::Data::new(AppState {
        config,
        video,
        voice,
        message,
        db: db_pool.clone(),
    });

    log::info!("Techub Comms Server starting on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("https://thbtechub.sbs")
            .allowed_origin("https://techub-comms.onrender.com")
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
            .app_data(web::Data::new(db_pool.clone()))
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
            .configure(routes::auth::configure)
            .configure(routes::video::configure)
            .configure(routes::voice::configure)
            .configure(routes::messages::configure)
            .configure(routes::webhooks::configure)
            // Static files with index file support
            .service(
                fs::Files::new("/", "./static")
                    .index_file("index.html")
                    .prefer_utf8(true)
            )
    })
    .bind(("0.0.0.0", port))?
    .workers(num_cpus::get().max(2))
    .run()
    .await
}
