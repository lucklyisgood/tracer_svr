pub mod config;
pub mod database;
pub mod log;
pub mod startup;

use std::future::Future;
use std::pin::Pin;

pub async fn main<F, T>(on_start: F, bind_func: startup::BindFunc)
where
    F: FnOnce() -> Pin<Box<dyn Future<Output = T>>>,
{
    let result = startup::startup(on_start, bind_func).await;
    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
