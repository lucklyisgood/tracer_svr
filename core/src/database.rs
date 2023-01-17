use sea_orm::{Database, DatabaseConnection, DbErr};
use super::config::APP_CFG;

async fn open_connection() -> Result<DatabaseConnection, DbErr> {
    let cfg = APP_CFG.get().expect("core: database get cfg fail when open connection");
    let opt = cfg.db.clone();
    let db = Database::connect(opt).await?;
    Ok(db)
}

pub async fn setup() -> Result<DatabaseConnection, DbErr> {
    open_connection().await
}