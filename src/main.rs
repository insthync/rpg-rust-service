#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate rbatis;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use rbatis::rbatis::Rbatis;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(";)")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read config
    dotenv().ok();
    let service_bind = env::var("SERVICE_BIND")
        .expect("`SERVICE_BIND` must be set");
    let database_url = env::var("DATABASE_URL")
        .expect("`DATABASE_URL` must be set");
        
    // Enable log crate to show sql logs
    fast_log::init_log("requests.log", log::Level::Info, None, true);
    // Initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
    let rb = Rbatis::new();
    // Connect to database  
    rb.link(&database_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(service_bind)?
    .run()
    .await
}