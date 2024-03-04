use yew::prelude::*;
use yew_router::prelude::Link;
use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use crate::{
    router::{
        Route,
        Language
    }
};

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct QA {
    question: String,
    answer: String,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct DataGen {
    #[serde(rename = "buttonText")]
    button_text: String,
    #[serde(rename = "QA")]
    qa: Vec<QA>,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Data {
    en: DataGen,
    fr: DataGen,
    de: DataGen,
    sp: DataGen,
    hi: DataGen,
    kr: DataGen,
    jp: DataGen,
    ru: DataGen,
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub lang: String,
}

#[allow(unused_variables)]
#[function_component(Home)]
pub fn home_page(props: &HomeProps) -> Html {
    let img_modes: Vec<&str> = vec!["light", "dark"];

    let data: UseStateHandle<Data> = use_state(|| Data::default());

    {
        let data = data.clone();
        use_effect_with((), move |_|  {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Data = Request::get("/json/home_page.json").send().await.unwrap().json().await.unwrap();
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

    let language = match props.lang.as_str() {
        "en" => Language::En,
        "fr" => Language::Fr,
        "de" => Language::De,
        "sp" => Language::Sp,
        "hi" => Language::Hi,
        "kr" => Language::Kr,
        "jp" => Language::Jp,
        "ru" => Language::Ru,
        _ => Language::En,
    };
    
    html! {
        <main class={classes!("home-page")}>
            <section>
                <figure class={classes!("hero-page")}>
                    {
                        (0..=3)
                            .map(|id| html!{<img class={classes!(format!("main-page-img-{}", id + 1))} src={format!("img/{}_mode_{}.png", img_modes[id % 2], ((id % 4) / 2) + 1)} draggable={"false"} />})
                            .collect::<Html>()
                    }
                </figure>
                <button>
                    <Link<Route> to={Route::GeneratorLang{lang: language}}>{&data_filtered.button_text}</Link<Route>>
                </button>
            </section>
            <section class={classes!("qa")}>
                {
                    data_filtered.qa.clone().into_iter().map(|qa| {
                        html! {
                            <article>
                                <h2>{qa.question}</h2>
                                <p>{qa.answer}</p>
                            </article>
                        }
                    }).collect::<Html>()
                }
            </section>
        </main>
    }
}
