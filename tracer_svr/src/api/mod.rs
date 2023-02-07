use actix_web::web;

pub mod svr;

pub fn bind_api<'a>(cfg: &'a mut web::ServiceConfig) {
    svr::bind_api(cfg);
}