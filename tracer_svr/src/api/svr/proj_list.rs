use actix_web::{web, put, Scope};

use core::typedefs::Response;
use core::database::db_rbatis::DB;
use crate::error::Error;
use crate::model::proj_list_model::ProjItem;
use crate::utils::send_ding_notice;

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
    let _ = send_ding_notice(
        "https://oapi.dingtalk.com/robot/send?access_token"
        , &serde_json::json!({
        "msgtype":"markdown",
        "markdown": {
            "title": "业务报警: test",
            "text": "> hahhahahhahaa"
        }
    })).await;
    Ok(Response::<()>::default())
}