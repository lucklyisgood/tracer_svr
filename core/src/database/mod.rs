#[cfg(feature="db")]
mod log_plugin;
#[cfg(feature="db")]
pub mod db_rbatis;

#[cfg(feature="db")]
pub async fn init_db() {
    let _ = db_rbatis::setup().await;
}

#[cfg(not(feature="db"))]
pub async fn init_db() {}

pub async fn setup() {
    init_db().await;
}

