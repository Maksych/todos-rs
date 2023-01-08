use frontend::app;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::Renderer::<app::App>::new().render();
}
