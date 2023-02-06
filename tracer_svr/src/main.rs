#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod basic;
mod api;
mod model;
use std::future::Future;
use std::pin::Pin;

async fn on_start2() {
    tracing::info!("bind api");
    model::setup().await;
}

#[tokio::main]
async fn main() {
    core::main(
        || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(on_start2())
        },
        api::bind_api,
    )
    .await;
}
