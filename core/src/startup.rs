use std::future::Future;
use std::pin::Pin;

use super::config::APP_CFG;
use super::{config, database};
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub type BindFunc = fn(&mut web::ServiceConfig);

async fn create_svr<F, F2, T>(on_start: F, bind_func: F2) -> std::io::Result<()>
where
    F: FnOnce() -> Pin<Box<dyn Future<Output = T>>>,
    F2: FnOnce(&mut web::ServiceConfig) + Clone + Send + Copy + 'static,
{
    let cfg = APP_CFG.get().expect("core: get cfg fail when create svr");
    database::setup().await.expect("open database fail");
    on_start().await;
    let svr_url = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!("starting server at {}", svr_url);
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(bind_func)
    })
    .bind(&svr_url)?
    .run()
    .await
}

pub async fn startup<F, T>(on_start: F, bind_func: BindFunc) -> std::io::Result<()>
where
    F: FnOnce() -> Pin<Box<dyn Future<Output = T>>>,
{
    config::setup().ok();
    let _guard = super::log::setup();
    create_svr(on_start, bind_func).await?;
    Ok(())
}
