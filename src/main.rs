use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod handlers;
mod db;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
dotenv().ok();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
db::establish_connection(&database_url);

HttpServer::new(|| {
App::new()
.wrap(middleware::error_handler())
.service(routes::product_routes())
})
.bind("127.0.0.1:8080")?
.run()
.await
}