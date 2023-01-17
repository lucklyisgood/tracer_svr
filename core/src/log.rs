use std::env;

pub fn setup() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt()
    .init();
}