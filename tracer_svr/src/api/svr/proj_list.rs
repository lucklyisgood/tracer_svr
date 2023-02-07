use actix_web::{web, put, Scope};

use core::typedefs::Response;
use core::database::db_rbatis::DB;
use crate::error::Error;
use crate::model::proj_list_model::ProjItem;

pub fn bind_api(scope: Scope) -> Scope {
    scope.service(create_proj)
}


#[put("/proj/create")]
pub async fn create_proj(proj_item: web::Json<ProjItem>) -> Result<Response<()>, Error> {
    if proj_item.name.is_empty() {
        return Err(Error::BadClientData("name is empty".into()))
    }
    if ProjItem::is_exist(&proj_item.name).await? {
        return Err(Error::CreateProjError("proj is exist".into()));
    }
    let  _ = ProjItem::create_proj(&DB.clone(), &proj_item).await?;
    Ok(Response::<()>::default())
}