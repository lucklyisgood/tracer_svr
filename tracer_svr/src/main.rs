use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

fn bind_api<'a>(cfg: &'a mut web::ServiceConfig) {
    cfg.service(hello);
}

fn main() {
    core::main(bind_api);
}
