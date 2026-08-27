use actix_web::{dev::Payload, Error, HttpResponse, Result};

pub fn error_handler() -> impl actix_web::middleware::Middleware {
actix_web::middleware::errhandlers::MiddlewareFactory::new(|err: &Error| HttpResponse::InternalServerError().body(format!("{:?}", err)))
}