use crate::components::icons::{Icon, Icons};
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct Button {
    src: String,
    width: String,
    alt: String,
    onclick: Callback<()>,
}

impl Button {
    pub fn new(src: Icons, width: String, alt: String, onclick: Callback<()>) -> Button {
        Button {
            src: Icon::new(src).icon,
            width,
            alt,
            onclick,
        }
    }

    pub fn create(&self) -> Html {
        let onclick = self.onclick.clone();
        html! {
            <button title={self.alt.clone()} onclick={move |_| {onclick.emit(());}}>
                <img src={self.src.clone()} width={self.width.clone()} alt={self.alt.clone()} />
            </button>
        }
    }
}
