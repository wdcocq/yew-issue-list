use yew_issue::App;

// trunk build
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().hydrate();
}
