use actix_web::{post, Responder};
use actix_web::{web, HttpResponse};
use k_scheduler::HelloRequest;
pub mod k_scheduler {
    tonic::include_proto!("k_scheduler");
}

pub fn buildDebugSvc(prefix: &str) -> actix_web::Scope {
    web::scope(prefix)
        .route("/hey", web::get().to(hey))
        .route("/echo", web::post().to(echo))
}

async fn echo(req_body: String, _config: web::Data<crate::Config>) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there, I am gary!!")
}
