use crate::auth::Claims;
use actix_service::{Service, Transform};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::error;
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll}; // Import the Claims struct from auth module

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
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();

                    let validation = Validation::default();
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret("your_secret_key".as_ref()),
                        &validation,
                    ) {
                        Ok(_token_data) => {
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?.map_into_left_body();
                                Ok(res)
                            });
                        }
                        Err(e) => {
                            error!("Token decode error: {:?}", e);
                            return Box::pin(async move {
                                let response = HttpResponse::Unauthorized()
                                    .json(json!({"error": "Unauthorized", "message": "Invalid token"}))
                                    .map_into_right_body();
                                Ok(req.into_response(response))
                            });
                        }
                    }
                }
            }
        }

        Box::pin(async move {
            let response = HttpResponse::Unauthorized()
                .json(json!({"error": "Unauthorized", "message": "Token missing"}))
                .map_into_right_body();
            Ok(req.into_response(response))
        })
    }
}
