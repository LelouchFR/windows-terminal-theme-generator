use yew::prelude::*;
use yew_hooks::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use gloo_console::log;
use stylist::{
    yew::Global
};
use crate::{
    colors::colors::{
        WindowsTerminalTheme,
    },
    components::tool_wrapper::ToolWrapper,
    utils::{
        WindowsTerminalText,
        ColoredTextHeader,
        generate_theme,
        gen_theme_from_link,
        color_classes
    }
};

#[derive(Debug, Clone)]
pub struct Button {
    src: String,
    width: String,
    alt: String,
    onclick: Callback<()>,
}

impl Button {
    pub fn new(src: String, width: String, alt: String, onclick: Callback<()>) -> Button {
        Button {src, width, alt, onclick}
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

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct DataValue {
    pub random: String,
    pub previous: String,
    pub next: String,
    pub modes: String,
    pub copy: String,
    pub share: String,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Data {
    pub en: DataValue,
    pub fr: DataValue,
    pub de: DataValue,
    pub sp: DataValue,
    pub hi: DataValue,
    pub kr: DataValue,
    pub jp: DataValue,
    pub ru: DataValue,
}

#[derive(Properties, PartialEq)]
pub struct GenProps {
    pub lang: String,
    #[prop_or_default]
    pub colors: String,
}

#[function_component(Generator)]
pub fn generator(props: &GenProps) -> Html {
    let clipboard = use_clipboard();
    let darkmode_active: UseStateHandle<bool> = use_state(|| false);
    let generated_theme: UseStateHandle<WindowsTerminalTheme> = use_state(|| if !props.colors.as_str().is_empty() {
        gen_theme_from_link(props.colors.clone())
    } else {
        generate_theme(*darkmode_active)
    });
    let previous_theme: UseStateHandle<WindowsTerminalTheme> = use_state(|| generate_theme(*darkmode_active));
    let next_theme: UseStateHandle<WindowsTerminalTheme> = use_state(|| generate_theme(*darkmode_active));
    let used_tools: Vec<String> = vec!["BrowserFetch".to_string(), "ColorTool".to_string()];


    // 3 states of a theme: 1. Live theme, 2. Previous theme, 3. Next Theme
    let generated_theme_clone = generated_theme.clone();
    let _previous_theme_clone = previous_theme.clone();
    let _next_theme_clone = next_theme.clone();
    let darkmode_active_clone = darkmode_active.clone();
    
    let randomize_theme_onclick = Callback::from(move |_| {
        let new_theme = generate_theme(*darkmode_active_clone);
        // previous_theme_clone.set(/* generated_theme */);
        generated_theme_clone.set(new_theme);
        log!("randomize button clicked");
    });
 
    // TODO
    let generated_theme_clone = generated_theme.clone();
    let previous_theme_onclick = Callback::from(move |_| {
        // previous_theme_clone.set(/* generated_theme_clone */);
        // next_theme_clone.set(/* previous_theme_clone */);
        log!("previous button clicked");
        log!(&*generated_theme_clone.to_json());
    });

    // TODO
    let next_theme_onclick = Callback::from(move |_| {
        // next_theme_clone.set();
        log!("next button clicked");
    });

    let darkmode_active_clone = darkmode_active.clone();
    let generate_theme_clone = generated_theme.clone();
    let mode_theme_onclick = Callback::from(move |_| {
        darkmode_active_clone.set(!*darkmode_active_clone);
        generate_theme_clone.set(generate_theme(!*darkmode_active_clone));
    });

    let clipboard_clone = clipboard.clone();
    let generated_theme_clone = generated_theme.clone();
    let copy_theme_onclick = Callback::from(move |_| {
        clipboard_clone.write_text(format!("{:?}", generated_theme_clone.to_json()).to_owned());
    });

    let clipboard_clone = clipboard.clone();
    let generated_theme_clone = generated_theme.clone();
    let lang = props.lang.clone();
    let share_theme_onclick = Callback::from(move |_| {
        log!("share link copied to clipboard!");
        let colors = generated_theme_clone.to_vec().iter().map(|x| format!("{}", &x[1..=x.len() - 1])).collect::<Vec<String>>();
        clipboard_clone.write_text(format!("https://windows-terminal-theme-generator.netlify.app/{}/generate/{}", lang, colors.join("-")));
    });
    
    let data: UseStateHandle<Data> = use_state(|| Data::default());

    {
        let data = data.clone();
        use_effect_with((), move |_|  {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Data = Request::get("/json/button_gen_text.json").send().await.unwrap().json().await.unwrap();
                data.set(fetched_data);
            });

            || ()
        });
    }

    let data_filtered = match props.lang.as_str() {
        "en" => &data.en,
        "fr" => &data.fr,
        "de" => &data.de,
        "sp" => &data.sp,
        "hi" => &data.hi,
        "kr" => &data.kr,
        "jp" => &data.jp,
        "ru" => &data.ru,
        _ => &data.en,
    };

    let button_info: Vec<(&str, &str, &str, Callback<()>)> = vec![
        ("/svg/random.svg", "50%", &data_filtered.random, randomize_theme_onclick.clone()),
        ("/svg/previous.svg", "50%", &data_filtered.previous, previous_theme_onclick.clone()),
        ("/svg/next.svg", "50%", &data_filtered.next, next_theme_onclick.clone()),
        ("/svg/moon.svg", "50%", &data_filtered.modes, mode_theme_onclick.clone()),
        ("/svg/copy.svg", "50%", &data_filtered.copy, copy_theme_onclick.clone()),
        ("/svg/share.svg", "50%", &data_filtered.share, share_theme_onclick.clone())
    ];
    
    let button_props: Vec<Button> = button_info.into_iter().map(|(icon, percent, label, action)| {
        Button::new(icon.to_string(), percent.to_string(), label.to_string(), action)
    }).collect();
    
    html! {
        <main class={classes!("generator")}>
            <Global css={generated_theme.get_css_vars()} />
            <ColoredTextHeader value={"Windows ,Terminal ,Theme ,Generator".to_string()} class={"".to_string()} />
            <section class={classes!("wt_official_example")}>
                <article>
                    <WindowsTerminalText />
                </article>
                <aside>
                    {
                        (0..=15)
                            .map(|i| html! {<div class={classes!(format!("{}{}", color_classes[i], "--as-background"))} title={color_classes[i]}></div>})
                            .collect::<Html>()
                    }
                </aside>
            </section>
            <section class={classes!("buttons")}>
                {
                    button_props
                        .into_iter()
                        .map(|button: Button| button.create())
                        .collect::<Html>()
                }
            </section>
            <ToolWrapper components={used_tools} lang={props.lang.clone()} />
        </main>
    }
}
