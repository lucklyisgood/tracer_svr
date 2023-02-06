use actix_web::{get, post, web, HttpResponse, Responder};
use super::model::human::Human;
use core::typedefs::{Response, Error};


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("rust actix web")
}

#[get("/human/get")]
async fn get_human() -> Result<impl Responder, Error> {
    let list: Vec<Human> = Human::select_by_column(&mut core::database::DB.clone(), "name", "vker").await?;
    Ok(Response::<Vec<Human>>::from(list))
}

#[post("human/new")]
async fn insert_human() -> Result<impl Responder, Error> {
    let p1 = Human{
        name: Some("vker".into()),
        id: None,
        create_time: None,
    };
    Human::insert(&mut core::database::DB.clone(), &p1).await?;
    Ok(Response::<()>::default())
}

pub fn bind_api<'a>(cfg: &'a mut web::ServiceConfig) {
    cfg.service(hello).service(get_human).service(insert_human);
}