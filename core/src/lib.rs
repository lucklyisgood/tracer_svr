pub mod config;
pub mod database;
pub mod log;
pub mod startup;
pub mod typedefs;

pub fn main(bind_func: startup::BindFunc) {
    let result = startup::startup(bind_func);
    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
