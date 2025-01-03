mod app;
mod components;
mod helpers;
mod plot;
mod xml;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
