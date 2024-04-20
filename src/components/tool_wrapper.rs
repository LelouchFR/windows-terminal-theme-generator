use yew::prelude::*;
use crate::{
    components::{
        ascii_browsers::{
            AsciiBrowser,
            Browsers,
            Charset,
            ThemeMode
        },
        colortool::ColorTool
    },
    router::Language
};

#[derive(Debug, PartialEq, Clone)]
pub enum ToolComponent {
    BrowserFetch,
    ColorTool,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ToolWrapperProps {
    pub components: Vec<ToolComponent>,
    pub lang: Language,
}

#[function_component(ToolWrapper)]
pub fn tool_wrapper(props: &ToolWrapperProps) -> Html {
    let tools: UseStateHandle<Vec<Html>> = use_state(|| {
        let mut initial_tools = Vec::new();

        for component in &props.components {
            let used_tool = match component {
                ToolComponent::BrowserFetch => html! {
                    <AsciiBrowser
                        browser_name={Browsers::Firefox}
                        browser_version={"0.1.0"}
                        screen_resolution={"1280x720"}
                        theme_mode={ThemeMode::Dark}
                        charset={Charset::UTF8}
                        user_language={props.lang.clone()}
                    />
                },
                ToolComponent::ColorTool => html!{<ColorTool />},
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
