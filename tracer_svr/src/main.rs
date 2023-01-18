mod api;
mod model;
use std::future::Future;
use std::pin::Pin;

async fn on_start2(state: core::typedefs::AppState) {
    tracing::info!("bind api");
    model::setup(&state).await;
}

#[tokio::main]
async fn main() {
    core::main(
        |state: core::typedefs::AppState| -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(on_start2(state))
        },
        api::bind_api,
    )
    .await;
}
