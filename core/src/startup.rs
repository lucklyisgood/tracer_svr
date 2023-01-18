use super::{config, database};
use super::config::APP_CFG;
use super::typedefs::AppState;
use actix_web::{HttpServer, middleware, App, web};

pub type BindFunc = fn(&mut web::ServiceConfig);

async fn create_svr(bind_func: BindFunc) -> std::io::Result<()> {
    let cfg = APP_CFG.get().expect("core: get cfg fail when create svr");
    let db = database::setup().await.expect("open database fail");
    let app_state = AppState{conn: db};

    let svr_url = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!("starting server at {}", svr_url);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::Logger::default())
            .configure(bind_func)
    })
    .bind(&svr_url)?
    .run()
    .await
}

#[tokio::main]
pub async fn startup(bind_func: BindFunc) -> std::io::Result<()> {
    config::setup().ok();
    let _guard = super::log::setup();
    create_svr(bind_func).await?;
    Ok(())
}