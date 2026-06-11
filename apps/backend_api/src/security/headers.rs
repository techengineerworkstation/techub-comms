use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::body::EitherBody;
use actix_web::Error;
use actix_web::http::header::{HeaderName, HeaderValue};
use futures::future::{ok, Ready, LocalBoxFuture};

pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static, B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service })
    }
}

pub struct SecurityHeadersMiddleware<S> { service: S }

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
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
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            let h = res.headers_mut();
            h.insert(HeaderName::from_static("x-content-type-options"), HeaderValue::from_static("nosniff"));
            h.insert(HeaderName::from_static("x-frame-options"), HeaderValue::from_static("DENY"));
            h.insert(HeaderName::from_static("x-xss-protection"), HeaderValue::from_static("1; mode=block"));
            h.insert(HeaderName::from_static("referrer-policy"), HeaderValue::from_static("strict-origin-when-cross-origin"));
            h.insert(HeaderName::from_static("permissions-policy"), HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"));
            h.insert(HeaderName::from_static("strict-transport-security"), HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"));
            h.insert(HeaderName::from_static("content-security-policy"), HeaderValue::from_static(
                "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' https://static.opentok.com; \
                 style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https:; \
                 connect-src 'self' https://video.api.vonage.com https://rest.nexmo.com wss://*.opentok.com; \
                 media-src 'self' blob:; font-src 'self' data:; frame-src 'self' https://*.opentok.com;"
            ));
            Ok(res.map_into_left_body())
        })
    }
}
