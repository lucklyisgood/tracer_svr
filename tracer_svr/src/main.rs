use core::actix_web;
use core::actix_web::{get, Responder, web::ServiceConfig};

#[get("/hello")]
async fn hello() -> impl Responder {
    "hello world"
}

fn bind_api(&mut cfg: ServiceConfig) {
    cfg.service(hello);
}

fn main() {
    core::main(bind_api);
}