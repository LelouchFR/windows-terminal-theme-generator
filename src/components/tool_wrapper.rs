use yew::prelude::*;
use crate::components::{
    ascii_browsers::AsciiBrowser,
    colortool::ColorTool
};
use gloo_console::log;

#[derive(Properties, PartialEq, Clone)]
pub struct ToolWrapperProps {
    pub components: Vec<String>,
    pub lang: String,
}

#[function_component(ToolWrapper)]
pub fn tool_wrapper(props: &ToolWrapperProps) -> Html {
    log!(format!("{:?}, {}", props.components, props.lang));
    
    let tools: UseStateHandle<Vec<Html>> = use_state(|| {
        let mut initial_tools = Vec::new();

        for component in &props.components {
            let used_tool = match component.as_str() {
                "BrowserFetch" => html!{<AsciiBrowser browser_name={"Firefox"} browser_version={"0.1.0"} screen_resolution={"1280x720"} theme_mode={"Dark"} charset={"UTF-8"} user_language={props.lang.clone()} />},
                "ColorTool" => html!{<ColorTool />},
                _ => html!{},
            };

            initial_tools.push(used_tool);
        }
        
        initial_tools
    });

    html! {
        <section class={classes!("bright-black--as-background")}>
            <button>{"WebFetch"}</button>
            <button>{"ColorTool"}</button>
            {(*tools).clone()}
        </section>
    }
}
