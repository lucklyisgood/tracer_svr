pub mod typedefs;
pub mod config;
pub mod log;
pub mod database;
pub mod startup;

pub use sea_orm;
pub use actix_web;

pub fn main(bind_func: startup::BindFunc) {
    let result = startup::startup(bind_func);
    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}