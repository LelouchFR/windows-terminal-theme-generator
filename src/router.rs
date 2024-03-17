use yew::prelude::*;
use yew_router::prelude::*;
use strum_macros::{EnumString, VariantNames, Display};
use crate::home_page::Home;
use crate::generate::Generator;

#[derive(Debug, Clone, PartialEq, EnumString, VariantNames, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    Fr,
    En,
    De,
    Sp,
    Hi,
    Kr,
    Jp,
    Ru,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/:lang")]
    HomeLang { lang: Language },
    #[at("/")]
    Home,
    #[at("/:lang/generate")]
    GeneratorLang { lang: Language },
    #[at("/generate")]
    Generator,
    #[at("/:lang/generate/:colors")]
    GeneratorLangWithColors { lang: Language, colors: String },
    #[at("/generate/:colors")]
    GeneratorWithColors { colors: String },
    #[at("/:lang/release")]
    ReleaseLang { lang: Language },
    #[at("/release")]
    Release,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::HomeLang { lang } => html! {<Home lang={lang.to_string()} />},
        Route::Home => html! {<Home lang={Language::En.to_string()} />},
        Route::GeneratorLang { lang } => html! {<Generator lang={lang.to_string()} />},
        Route::Generator => html! {<Generator lang={Language::En.to_string()} />},
        Route::GeneratorLangWithColors { lang, colors } => html! {<Generator lang={lang.to_string()} colors={colors} />},
        Route::GeneratorWithColors { colors } => html! {<Generator lang={Language::En.to_string()} colors={colors} />},
        Route::ReleaseLang { lang } => html! {<h1>{"V 1.0 in "}{lang.to_string()}</h1>},
        Route::Release => html! {<h1>{"V 1.0 in "}{Language::En.to_string()}</h1>},
        Route::NotFound => html! { <h1>{ "404: Page not found" }</h1> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
