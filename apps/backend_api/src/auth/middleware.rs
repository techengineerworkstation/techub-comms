use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::body::EitherBody;
use actix_web::Error;
use futures::future::{ok, Ready, LocalBoxFuture};
use sqlx::PgPool;

pub struct AuthMiddleware {
    pub pool: PgPool,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static, B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner { service, pool: self.pool.clone() })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
    pool: PgPool,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static, B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let pool = self.pool.clone();
        let path = req.path().to_string();

        let skip_paths = ["/api/auth/login", "/api/auth/register", "/health"];
        let is_static = path.starts_with("/web_leptos-") || path.ends_with(".js")
            || path.ends_with(".wasm") || path.ends_with(".css") || path.ends_with(".svg")
            || path == "/style.css" || path == "/animations.css" || path == "/favicon.svg"
            || path == "/" || path == "/index.html";

        if skip_paths.contains(&path.as_str()) || is_static || path.starts_with("/api/") {
            let fut = self.service.call(req);
            return Box::pin(async move {
                fut.await.map(|r| r.map_into_left_body())
            });
        }

        let token = extract_token(&req);
        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(token) = token {
                match crate::auth::get_user_from_token(&pool, &token).await {
                    Ok((_user_id, email, name)) => {
                        log::info!("Auth: {} ({})", name, email);
                    }
                    Err(e) => {
                        log::warn!("Auth failed: {}", e);
                    }
                }
            }
            fut.await.map(|r| r.map_into_left_body())
        })
    }
}

fn extract_token(req: &ServiceRequest) -> Option<String> {
    if let Some(auth) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    if let Some(cookie) = req.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            for part in cookie_str.split(';') {
                let part = part.trim();
                if part.starts_with("techub_token=") {
                    return Some(part[13..].to_string());
                }
            }
        }
    }

    None
}
