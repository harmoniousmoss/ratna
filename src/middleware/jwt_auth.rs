use actix_service::{Service, Transform};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll};

#[derive(Debug, Deserialize)]
struct Claims;

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();

                    let validation = Validation::default();
                    if let Ok(_token_data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret("your_secret_key".as_ref()),
                        &validation,
                    ) {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?.map_into_left_body();
                            Ok(res)
                        });
                    }
                }
            }
        }

        Box::pin(async move {
            let response = HttpResponse::Unauthorized()
                .json(json!({"error": "Unauthorized", "message": "Token missing or invalid"}))
                .map_into_right_body();
            Ok(req.into_response(response))
        })
    }
}
