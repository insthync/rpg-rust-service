#[macro_use]
extern crate dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(";)")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let service_bind = env::var("SERVICE_BIND")
        .expect("SERVICE_BIND must be set");
        
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(service_bind)?
    .run()
    .await
}