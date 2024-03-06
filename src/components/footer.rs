use yew::prelude::*;
use yew_router::prelude::Link;
use crate::{
    router::{
        Route,
        Language
    },
    utils::get_random_color_on_hover
};

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub lang: String,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let repository_link: String = "https://github.com/LelouchFR/windows-terminal-theme-generator".to_string();
    let random_hover_color: UseStateHandle<String> = use_state(|| get_random_color_on_hover());

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

    let on_mouse_hover =  {
        let random_hover_color = random_hover_color.clone();
        move |_| random_hover_color.set(get_random_color_on_hover())
    };
    
    html! {
        <footer>
            <section class={classes!("footer_columns")}>
                <section>
                    <h3>{"Github"}</h3>
                    <ul>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={repository_link.clone()}>{"Repository"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={format!("{}/issues", repository_link.clone())}>{"Issues"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={format!("{}/pulls", repository_link.clone())}>{"Pull Requests"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={format!("{}/commits/main/", repository_link.clone())}>{"Commits"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={format!("{}/graphs/contributors", repository_link.clone())}>{"Contributors"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"https://github.com/LelouchFR"}>{"Maintainer"}</a></li>
                    </ul>
                </section>
                <section>
                    <h3>{"Languages"}</h3>
                    <ul>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::En}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"English"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::Fr}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Français"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::De}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Deutsch"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::Sp}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Español"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::Hi}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Hindi"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::Kr}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Korean"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::Jp}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Japanese"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::HomeLang{lang: Language::Ru}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Russian"}</span></Link<Route>></li>
                    </ul>
                </section>
                <section>
                    <h3>{"Routes"}</h3>
                    <ul>
                        <li><Link<Route> to={Route::HomeLang{lang: language.clone()}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Home"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::GeneratorLang{lang: language.clone()}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Generate"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::ReleaseLang{lang: language.clone()}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Releases"}</span></Link<Route>></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"/sitemap.xml"}>{"Sitemap"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"/robots.txt"}>{"Robot file"}</a></li>
                    </ul>
                </section>
                <section>
                    <h3>{"Contact"}</h3>
                    <ul>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"https://github.com/LelouchFR/windows-terminal-theme-generator/issues/new?assignees=LelouchFR&labels=bug&projects=&template=bug_report.yml&title=%5BBUG%5D%3A+"}>{"Report a Bug"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"https://github.com/LelouchFR/windows-terminal-theme-generator/issues/new?assignees=&labels=feature&projects=&template=feature_request.yml&title=%5BFEAT%5D%3A+"}>{"Feature Request"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"https://github.com/sponsors/LelouchFR"}>{"Support the Project"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"mailto:dev.baptiste.zahnow@gmail.com"}>{"Contact Us"}</a></li>
                    </ul>
                </section>
            </section>
            <section class={classes!("license")}>
                <p><a href={"https://github.com/LelouchFR/windows-terminal-theme-generator/blob/main/LICENSE"} onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"© 2024 Baptiste Zahnow. Apache V2.0 License."}</a></p>
            </section>
        </footer>
    }
}
