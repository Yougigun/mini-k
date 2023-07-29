#![allow(non_snake_case, unused_imports)]
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use k_scheduler::greeter_client::GreeterClient;
use k_scheduler::HelloRequest;
use log::info;

pub mod k_scheduler {
    tonic::include_proto!("k_scheduler");
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String, config: web::Data<Config>) -> impl Responder {
    let mut client = GreeterClient::connect(config.k_scheduler_url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(HelloRequest { name: req_body });

    let response = client.say_hello(request).await.unwrap();

    HttpResponse::Ok().body(response.into_inner().message)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
