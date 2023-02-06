use once_cell::sync::OnceCell;
use std::env;
use dotenv::dotenv;

pub static APP_CFG: OnceCell<AppConfig> = OnceCell::new();

#[derive(Debug)]
pub enum LogRollingType {
    NEVER,
    DAILY,
    HOURLY,
    MINUTELY,
}

#[derive(Debug)]
pub struct LogConfig {
    pub rolling_type: LogRollingType,
    pub log_prefix_path: String,
    pub log_prefix_name: String,
}

#[derive(Debug)]
pub struct DBConfig {
    pub url: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub log: LogConfig,
    pub db: DBConfig,
}

pub fn create_log_cfg() -> LogConfig {
    let rolling_type = env::var("LOG_ROLLING_TYPE").unwrap_or("daily".into());
    let rolling_type = match rolling_type.as_str() {
        "daily" => LogRollingType::DAILY,
        "hourly" => LogRollingType::HOURLY,
        "minutely" => LogRollingType::MINUTELY,
        _ => LogRollingType::NEVER,
    };
    let log_prefix_path = env::var("LOG_PREFIX_PATH").unwrap_or("./logs".into());
    let log_prefix_name = env::var("LOG_PREFIX_FILE_NAME").unwrap_or("log".into());
    LogConfig {
        rolling_type,
        log_prefix_path,
        log_prefix_name,
    }
}

pub fn create_db_cfg() -> DBConfig {
    let database_url = env::var("DATABASE_URL").unwrap_or("".into());
    DBConfig { url: database_url }
}

pub fn setup() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    
    let log = create_log_cfg();
    let db = create_db_cfg();

    APP_CFG
        .set(AppConfig { host, port, log, db })
        .expect("App Config set err");
    Ok(())
}
