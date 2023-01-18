use once_cell::sync::OnceCell;
use sea_orm::ConnectOptions;
use std::{env, time::Duration};
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
    pub sqlx_log_enable: bool,
    pub sqlx_log_level: log::LevelFilter,
}

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub db: ConnectOptions,
    pub log: LogConfig,
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

    let sqlx_log_enable = env::var("SQLX_LOG_ENABLE").unwrap_or("false".into());
    let sqlx_log_enable = match sqlx_log_enable.as_str() {
        "true" => true,
        _ => false,
    };
    let sqlx_log_level = env::var("SQLX_LOG_LEVEL").unwrap_or("debug".into());
    let sqlx_log_level = match sqlx_log_level.as_str() {
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    };
    LogConfig {
        rolling_type,
        log_prefix_path,
        log_prefix_name,
        sqlx_log_enable,
        sqlx_log_level,
    }
}

pub fn create_db_cfg() -> ConnectOptions {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite::memory:".into());
    let max_connections: u32 = env::var("MAX_CONNECTIONS").unwrap_or("100".into()).parse().unwrap_or(100);
    let min_connections: u32 = env::var("MIN_CONNECTIONS").unwrap_or("5".into()).parse().unwrap_or(5);
    let connect_timeout: u64 = env::var("CONNECT_TIMEOUT").unwrap_or("8000".into()).parse().unwrap_or(8000);
    let acquire_timeout: u64 = env::var("ACQUIRE_TIMEOUT").unwrap_or("8000".into()).parse().unwrap_or(8000);
    let idle_timeout: u64 = env::var("IDLE_TIMEOUT").unwrap_or("8000".into()).parse().unwrap_or(8000);
    let max_lifetime: u64 = env::var("MAX_LIFETIME").unwrap_or("8000".into()).parse().unwrap_or(8000);
    let mut opts = ConnectOptions::new(db_url);
    opts
    .max_connections(max_connections)
    .min_connections(min_connections)
    .connect_timeout(Duration::from_millis(connect_timeout))
    .acquire_timeout(Duration::from_millis(acquire_timeout))
    .idle_timeout(Duration::from_millis(idle_timeout))
    .max_lifetime(Duration::from_millis(max_lifetime));

    opts
}

pub fn setup() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let mut db = create_db_cfg();
    let log = create_log_cfg();
    db.sqlx_logging(log.sqlx_log_enable).sqlx_logging_level(log.sqlx_log_level);

    APP_CFG
        .set(AppConfig { host, port, db, log })
        .expect("App Config set err");
    Ok(())
}
