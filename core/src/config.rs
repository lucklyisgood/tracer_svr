use std::env;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::ConnectOptions;

pub static APP_CFG: OnceCell<AppConfig> = OnceCell::new();

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub db: ConnectOptions,
}

pub fn create_db_cfg() -> ConnectOptions {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let max_connections: u32 = env::var("MAX_CONNECTIONS").expect("").parse().expect("");
    let mut opts = ConnectOptions::new(db_url);
    opts.max_connections(max_connections);
    opts
}

pub fn setup() -> std::io::Result<()> {
    dotenv().ok();
    
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let db = create_db_cfg();

    APP_CFG.set(AppConfig {host, port, db}).expect("App Config set err");
    Ok(())
}