use std::env;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::config::{APP_CFG, LogRollingType};

pub fn setup() -> WorkerGuard {
    if cfg!(debug_assertions) {
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
    } else {
        env::set_var("RUST_LOG", "info");
    }

    init_tracing()
}

fn create_rolling_file_appender() -> RollingFileAppender {
    let cfg = APP_CFG.get().unwrap();
    let log_cfg = &cfg.log;
    match log_cfg.rolling_type {
        LogRollingType::NEVER => tracing_appender::rolling::never(&log_cfg.log_prefix_path, &log_cfg.log_prefix_name),
        LogRollingType::DAILY => tracing_appender::rolling::daily(&log_cfg.log_prefix_path, &log_cfg.log_prefix_name),
        LogRollingType::HOURLY => tracing_appender::rolling::hourly(&log_cfg.log_prefix_path, &log_cfg.log_prefix_name),
        LogRollingType::MINUTELY => tracing_appender::rolling::minutely(&log_cfg.log_prefix_path, &log_cfg.log_prefix_name),
    }
}

fn init_tracing() -> WorkerGuard {
    let file_appender = create_rolling_file_appender();
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            // subscriber configuration
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::CLOSE)
            .finish()
            .with(
                fmt::Layer::default()
                .with_span_events(FmtSpan::CLOSE)
                .with_writer(file_writer)
            )
            // add additional writers
    ).expect("Unable to set global tracing subscriber");
    // tracing
    tracing::info!("{}", EnvFilter::from_default_env());
    tracing::debug!("tracing initialized.");
    guard
}