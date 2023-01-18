pub mod config;
pub mod database;
pub mod log;
pub mod startup;
pub mod typedefs;

pub use sea_orm;

pub fn main(on_start: startup::OnStartFunc, bind_func: startup::BindFunc) {
    let result = startup::startup(on_start, bind_func);
    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
