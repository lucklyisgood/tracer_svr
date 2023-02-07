use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::Rbatis;
use rbatis::py_sql;
use serde_json::Value;
use crate::error::Error;
use core::database::db_rbatis::DB;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjItem {
    pub id: Option<u32>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
    pub desc: Option<String>,
    pub avatar: Option<String>,
    pub name: String,
}

impl ProjItem {
    #[py_sql("
` INSERT INTO proj_list `
` (name, desc, avatar) `
` VALUES `
` (#{proj_item.name}, #{proj_item.desc}, #{proj_item.avatar}) `
    ;")]
    pub async fn create_proj(db: &Rbatis, proj_item: &ProjItem) -> rbatis::Result<()> {
        impled!()
    }

    #[py_sql("
` SELECT name `
` FROM proj_list `
` WHERE name = #{name} `
    ;")]
    async fn _find_proj(db: &Rbatis, name: &str) -> rbatis::Result<Vec<Value>> {
        impled!()
    }

    pub async fn is_exist(name: &str) -> Result<bool, Error> {
        let db = &mut DB.clone();
        let list = ProjItem::_find_proj(db, name).await?;
        if list.len() <= 0 {
            return Ok(false)
        }
        Ok(true)
    }
}