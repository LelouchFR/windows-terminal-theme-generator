mod home_page;
mod generate;
mod router;
mod components;
mod colors;
mod utils;

fn main() {
    yew::Renderer::<router::Router>::new().render();
}
