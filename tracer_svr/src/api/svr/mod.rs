use actix_web::web;
mod proj_list;

pub fn bind_api<'a>(cfg: &'a mut web::ServiceConfig) {
    let mut scope = web::scope("/svr");
    scope = proj_list::bind_api(scope);
    cfg.service(scope);
}