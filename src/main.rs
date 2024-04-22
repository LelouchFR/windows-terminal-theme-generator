mod colors;
mod components;
mod generate;
mod home_page;
mod router;
mod utils;

fn main() {
    yew::Renderer::<router::Router>::new().render();
}
