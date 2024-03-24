use yew::prelude::*;
use crate::components::icons::{Icon, Icons};

#[derive(Default)]
pub enum AlertMode {
    Warning,
    Error,
    Validation,
    Info,
    #[default]
    None
}

pub struct AlertItems {
    pub color: String,
    pub icon: String,
}

#[derive(Default)]
pub struct Alert {
    pub mode: AlertMode,
    pub time_duration: u8,
    pub text: String,
}

impl Alert {
    pub fn new(mode: AlertMode, time_duration: u8, text: String) -> Self {
        Alert {
            mode, time_duration, text
        }
    }

    pub fn create(&self) -> Html {
        let items: AlertItems = match self.mode {
            AlertMode::Warning => AlertItems {
                color: "yellow--as-background".to_string(),
                icon: Icon::new(Icons::Warning).icon
            },
            AlertMode::Error => AlertItems {
                color: "red--as-background".to_string(),
                icon: Icon::new(Icons::Error).icon
            },
            AlertMode::Validation => AlertItems {
                color: "green--as-background".to_string(),
                icon: Icon::new(Icons::Validation).icon 
            },
            AlertMode::Info => AlertItems {
                color: "blue--as-background".to_string(),
                icon: Icon::new(Icons::Info).icon
            },
            AlertMode::None => AlertItems {
                color: "".to_string(),
                icon: "".to_string()
            },
        };

        let time_class: String = format!("time-{}s", self.time_duration);

        html! {
            {
                if items.color.is_empty() && items.icon.is_empty() {
                    html!{}
                } else {
                    html! {
                        <div class={classes!("alert", items.color, time_class)}>
                            <figure>
                                <img src={items.icon} draggable={"false"} width="50px" />
                                <figcaption class={classes!("alert-text", "background")}>{self.text.clone()}</figcaption>
                            </figure>
                        </div>
                    }
                }
            }
        }
    }
}
