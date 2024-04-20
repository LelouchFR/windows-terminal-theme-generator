use yew::prelude::*;
use yew_hooks::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use gloo_console::log;
use stylist::yew::Global;
use crate::{
    colors::colors::WindowsTerminalTheme,
    components::{
        tool_wrapper::{ToolComponent, ToolWrapper},
        alerts::{AlertMode, Alert},
        buttons::Button,
        icons::Icons
    },
    utils::{
        WindowsTerminalText,
        ColoredTextHeader,
        generate_theme,
        gen_theme_from_link,
        color_classes
    },
    router::Language
};

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
    pub lang: Language,
    #[prop_or_default]
    pub colors: String,
}

#[function_component(Generator)]
pub fn generator(props: &GenProps) -> Html {
    let clipboard = use_clipboard();
    let darkmode_active: UseStateHandle<bool> = use_state(|| false);

    let alerts: UseStateHandle<Alert> = use_state(|| {
        if !props.colors.trim().is_empty() {
            if props.colors.split('-').count() == 20 {
                Alert::new(AlertMode::Validation, 10, "All Colors are Imported with Success".to_string())
            } else {
                Alert::new(AlertMode::Error, 10, format!("Not Enough Color For a Complete Theme, There Should be 20, instead found {} colors", props.colors.split('-').count()))
            }
        } else {
            Alert::default()
        }
    });

    let generated_theme: UseStateHandle<WindowsTerminalTheme> = use_state(|| if !props.colors.as_str().is_empty() {
        gen_theme_from_link(props.colors.clone())
    } else {
        generate_theme(*darkmode_active)
    });
    // let previous_theme: UseStateHandle<WindowsTerminalTheme> = use_state(|| generate_theme(*darkmode_active));
    // let next_theme: UseStateHandle<WindowsTerminalTheme> = use_state(|| generate_theme(*darkmode_active));
    let used_tools: Vec<ToolComponent> = vec![ToolComponent::BrowserFetch, ToolComponent::ColorTool];


    // 3 states of a theme: 1. Live theme, 2. Previous theme, 3. Next Theme
    let generated_theme_clone = generated_theme.clone();
    let darkmode_active_clone = darkmode_active.clone();
    let alerts_clone = alerts.clone();
    let randomize_theme_onclick = Callback::from(move |_| {
        let new_theme = generate_theme(*darkmode_active_clone);
        generated_theme_clone.set(new_theme);
        alerts_clone.set(Alert::default());
    });
 
    // TODO
    let alerts_clone = alerts.clone();
    let previous_theme_onclick = Callback::from(move |_| {
        alerts_clone.set(Alert::new(AlertMode::Warning, 5, "Previous Button not implemented For now".to_string()));
    });

    // TODO
    let alerts_clone = alerts.clone();
    let next_theme_onclick = Callback::from(move |_| {
        alerts_clone.set(Alert::new(AlertMode::Warning, 5, "Next Button not implemented For now".to_string()));
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
    let alerts_clone = alerts.clone();
    let copy_theme_onclick = Callback::from(move |_| {
        clipboard_clone.write_text(format!("{:?}", generated_theme_clone.to_json()).to_owned());
        alerts_clone.set(Alert::new(AlertMode::Info, 5, "Json Copied with Success".to_string()));
    });

    let clipboard_clone = clipboard.clone();
    let generated_theme_clone = generated_theme.clone();
    let lang = props.lang.clone();
    let alerts_clone = alerts.clone();
    let share_theme_onclick = Callback::from(move |_| {
        let colors = generated_theme_clone.to_vec().iter().map(|x| format!("{}", &x[1..=x.len() - 1])).collect::<Vec<String>>();
        clipboard_clone.write_text(format!("https://windows-terminal-theme-generator.netlify.app/{}/generate/{}", lang, colors.join("-")));
        alerts_clone.set(Alert::new(AlertMode::Info, 5, "Link Copied with Success".to_string()));
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

    let data_filtered = match props.lang {
        Language::En => &data.en,
        Language::Fr => &data.fr,
        Language::De => &data.de,
        Language::Sp => &data.sp,
        Language::Hi => &data.hi,
        Language::Kr => &data.kr,
        Language::Jp => &data.jp,
        Language::Ru => &data.ru,
    };

    let button_info: Vec<(Icons, &str, &str, Callback<()>)> = vec![
        (Icons::Random, "50%", &data_filtered.random, randomize_theme_onclick.clone()),
        (Icons::Previous, "50%", &data_filtered.previous, previous_theme_onclick.clone()),
        (Icons::Next, "50%", &data_filtered.next, next_theme_onclick.clone()),
        (Icons::Theme, "50%", &data_filtered.modes, mode_theme_onclick.clone()),
        (Icons::Copy, "50%", &data_filtered.copy, copy_theme_onclick.clone()),
        (Icons::Share, "50%", &data_filtered.share, share_theme_onclick.clone())
    ];
    
    let button_props: Vec<Button> = button_info.into_iter().map(|(icon, percent, label, action)| {
        Button::new(icon, percent.to_string(), label.to_string(), action)
    }).collect();
    
    html! {
        <main class={classes!("generator")}>
            <Global css={generated_theme.get_css_vars()} />
            <ColoredTextHeader value={"Windows ,Terminal ,Theme ,Generator".to_string()} class={"".to_string()} />
            {alerts.clone().create()}
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
