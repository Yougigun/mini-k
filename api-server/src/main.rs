#![allow(non_snake_case, unused_imports)]
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;

pub mod service;

pub mod k_scheduler {
    tonic::include_proto!("k_scheduler");
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!")
}
#[derive(Clone)]
pub struct Config {
    pub k_scheduler_url: String,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let PORT = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let KSCHEDULER_URL = std::env::var("KSCHEDULER_URL").unwrap();

    let config = web::Data::new(Config {
        k_scheduler_url: KSCHEDULER_URL,
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %b(bytes) %D(ms)"))
            .app_data(config.clone())
            .service(service::buildContainerScope("/container"))
            .service(service::buildDebugSvcScope("/debug"))
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
