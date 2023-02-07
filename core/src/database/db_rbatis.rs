use rbatis::Rbatis;
use rbatis::dark_std::sync::SyncVec;
use rbatis::rbdc::Error;
use rbatis::RbatisOption;

use once_cell::sync::Lazy;

use crate::config::APP_CFG;
use std::path::Path;


use super::log_plugin::RbatisTracingLogPlugin;

#[cfg(feature="sqlite")]
use rbdc_sqlite::driver::SqliteDriver;

pub static DB: Lazy<Rbatis> = Lazy::new(|| {Rbatis::new_with_opt(RbatisOption{
    sql_intercepts: SyncVec::new(),
    log_plugin: Box::new(RbatisTracingLogPlugin::default()),
})});

fn create_sqlite_dir(url: &str) {
    if url.starts_with("sqlite://") {
        let file_path = url.replace("sqlite://", "");
        let p = Path::new(&file_path).parent().expect("sqlite url err");
        std::fs::create_dir_all(p).expect("sqlite create parent dir fail!");
    }
}

pub async fn setup() -> Result<(), Error> {
    let cfg = APP_CFG.get().expect("core: database get cfg fail when open connection");
    create_sqlite_dir(&cfg.db.url);
    Ok(())
}

#[cfg(feature="sqlite")]
pub async fn init_sqlite() -> Result<(), Error> {
    let cfg = APP_CFG.get().expect("core: database get cfg fail when open connection");
    DB.init(SqliteDriver{}, &cfg.db.url)?;
    Ok(())
}