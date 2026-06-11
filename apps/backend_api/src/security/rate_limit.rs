use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::body::EitherBody;
use actix_web::Error;
use futures::future::{ok, Ready, LocalBoxFuture};

pub struct RateLimiter {
    window: Duration,
    max_requests: u32,
}

impl RateLimiter {
    pub fn new(window_secs: u64, max_requests: u32) -> Self {
        Self { window: Duration::from_secs(window_secs), max_requests }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static, B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimiterMiddleware {
            service, window: self.window, max_requests: self.max_requests,
            state: Mutex::new(HashMap::new()),
        })
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    window: Duration,
    max_requests: u32,
    state: Mutex<HashMap<String, (u32, Instant)>>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();
        let entry = state.entry(ip.clone()).or_insert((0, now));
        if now.duration_since(entry.1) > self.window { entry.0 = 0; entry.1 = now; }
        entry.0 += 1;
        if entry.0 > self.max_requests {
            log::warn!("[RateLimit] {} exceeded limit", ip);
            let resp = req.into_response(
                actix_web::HttpResponse::TooManyRequests()
                    .insert_header(("Retry-After", self.window.as_secs().to_string()))
                    .json(serde_json::json!({ "error": "Rate limit exceeded" }))
            ).map_into_right_body();
            return Box::pin(async { Ok(resp) });
        }
        drop(state);
        let fut = self.service.call(req);
        Box::pin(async move {
            fut.await.map(|r| r.map_into_left_body())
        })
    }
}
