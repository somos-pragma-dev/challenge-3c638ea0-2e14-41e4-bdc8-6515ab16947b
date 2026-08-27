use actix_web::{web, HttpResponse, Responder};
use crate::handlers::product_handler::*;

pub fn product_routes(cfg: &mut web::ServiceConfig) {
cfg.service(web::resource("/products").route(web::post().to(create_product)))
.service(web::resource("/products/{id}").route(web::get().to(get_product))
.route(web::put().to(update_product))
.route(web::delete().to(delete_product)));
}