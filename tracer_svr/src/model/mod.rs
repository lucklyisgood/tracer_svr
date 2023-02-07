use tracing;

#[cfg(feature="sqlite")]
use core::database::db_rbatis::init_sqlite;

use core::database::db_rbatis::DB;

use regex::Regex;
use once_cell::sync::Lazy;
use crate::error::Error;

pub mod sql;
pub mod proj_list_model;

use sql::sync_table;

#[cfg(feature="sqlite")]
async fn init_db() {
    init_sqlite().await.unwrap();
}

pub async fn exec_sql(table_name: &str, sql: &str) -> Result<(), Error> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<TABLE_NAME>").unwrap());
    let raw_sql = RE.replace_all(sql, table_name).into_owned();
    DB.exec(&raw_sql, vec![]).await?;
    Ok(())
}

pub async fn setup() {
    tracing::info!("model setup");
    init_db().await;
    sync_table().await;
}
