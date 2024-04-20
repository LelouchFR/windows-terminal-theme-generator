use yew::prelude::*;
use yew_router::prelude::Link;
use gloo_net::http::Request;
use serde::Deserialize;
use crate::{
    router::{
        Route,
        Language
    },
    components::footer::Footer,
    utils::ColoredTextHeader
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
    pub lang: Language,
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
    
    html! {
        <>
            <ColoredTextHeader value={"Windows ,Terminal ,Theme ,Generator".to_string()} class={"home-page-title".to_string()} />
            <main class={classes!("home-page")}>
                <section>
                    <figure class={classes!("hero-page")}>
                        {
                            (0..=3)
                                .map(|id| html!{<img class={classes!(format!("main-page-img-{}", id + 1))} alt={"an example image from the generator"} src={format!("img/{}_mode_{}.png", img_modes[id % 2], ((id % 4) / 2) + 1)} draggable={"false"} />})
                                .collect::<Html>()
                        }
                    </figure>
                    <button>
                        <Link<Route> to={Route::GeneratorLang{lang: props.lang.clone()}}>{&data_filtered.button_text}</Link<Route>>
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
            <Footer lang={props.lang.clone()}/>
        </>
    }
}
