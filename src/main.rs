mod app;
mod components;
use components:: {
    board::Board,
};
use app::App;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
