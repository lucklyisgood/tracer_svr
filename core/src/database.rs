use sea_orm::{Database, DatabaseConnection, DbErr};
use super::config::APP_CFG;
use std::path::Path;

fn create_sqlite_dir(url: &str) {
    if url.starts_with("sqlite://") {
        let file_path = url.replace("sqlite://", "");
        let p = Path::new(&file_path).parent().expect("sqlite url err");
        std::fs::create_dir_all(p).expect("sqlite create parent dir fail!");
    }
}

async fn open_connection() -> Result<DatabaseConnection, DbErr> {
    let cfg = APP_CFG.get().expect("core: database get cfg fail when open connection");
    create_sqlite_dir(cfg.db.clone().get_url());
    let db = Database::connect(cfg.db.clone()).await?;
    Ok(db)
}

pub async fn setup() -> Result<DatabaseConnection, DbErr> {
    open_connection().await
}