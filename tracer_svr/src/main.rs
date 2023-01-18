mod api;
mod model;

fn on_start(state: &core::typedefs::AppState) {
    // sync table
    model::setup(state);
}

fn main() {
    core::main(on_start, api::bind_api);
}
