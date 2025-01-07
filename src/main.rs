mod app;
mod components;
mod helpers;
mod macros;
mod map;
mod plot;
mod xml;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
