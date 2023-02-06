use actix_web::{get, web, HttpResponse, Responder};
use std::thread;
use std::time::Duration;
use tracing;
use tracing::Level;


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub fn bind_api<'a>(cfg: &'a mut web::ServiceConfig) {
    cfg.service(hello);
}