use crate::{
    router::{Language, Route},
    utils::get_random_color_on_hover,
};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub lang: Language,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    #[rustfmt::skip]
    let repository_link: String = "https://github.com/LelouchFR/windows-terminal-theme-generator".to_string();
    let random_hover_color: UseStateHandle<String> = use_state(|| get_random_color_on_hover());

    let on_mouse_hover = {
        let random_hover_color = random_hover_color.clone();
        move |_| random_hover_color.set(get_random_color_on_hover())
    };
    
    html! {
        <footer>
            <section class={classes!("footer_columns")}>
                <section>
                    <h3>{"Github"}</h3>
                    <ul>
                        {
                            vec![repository_link.to_string().clone(), format!("{}/issues", repository_link.clone()).to_string(), format!("{}/pulls", repository_link.clone()).to_string(), format!("{}/commits/main/", repository_link.clone()).to_string(), format!("{}/graphs/contributors", repository_link.clone()).to_string(), "https://github.com/LelouchFR".to_string()]
                                .iter()
                                .enumerate()
                                .map(|(i, link)| {
                                    let texts: Vec<&str> = vec!["Repository", "Issues", "Pull Requests", "Commits", "Contributors", "Maintainer"];

                                    html! {
                                        <li>
                                            <a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={link.clone()}>
                                                {texts[i]}
                                            </a>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </ul>
                </section>
                <section>
                    <h3>{"Languages"}</h3>
                    <ul>
                        {
                            vec![Language::En, Language::Fr, Language::De, Language::Sp, Language::Hi, Language::Kr, Language::Jp, Language::Ru]
                                .iter()
                                .enumerate()
                                .map(|(i, lang)| {
                                    let texts: Vec<&str> = vec!["English", "Français", "Deutsch", "Español", "Hindi", "Korean", "Japanese", "Russian"];

                                    html! {
                                        <li>
                                            <Link<Route> to={Route::HomeLang{lang: lang.clone()}}>
                                                <span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>
                                                    {texts[i]}
                                                </span>
                                            </Link<Route>>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </ul>
                </section>
                <section>
                    <h3>{"Routes"}</h3>
                    <ul>
                        <li><Link<Route> to={Route::HomeLang{lang: props.lang.clone()}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Home"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::GeneratorLang{lang: props.lang.clone()}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Generate"}</span></Link<Route>></li>
                        <li><Link<Route> to={Route::ReleaseLang{lang: props.lang.clone()}}><span onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"Releases"}</span></Link<Route>></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"/sitemap.xml"}>{"Sitemap"}</a></li>
                        <li><a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={"/robots.txt"}>{"Robot file"}</a></li>
                    </ul>
                </section>
                <section>
                    <h3>{"Contact"}</h3>
                    <ul>
                        {
                            vec![format!("{}/issues/new?assignees=LelouchFR&labels=bug&projects=&template=bug_report.yml&title=%5BBUG%5D%3A+", repository_link.clone()), format!("{}/issues/new?assignees=&labels=feature&projects=&template=feature_request.yml&title=%5BFEAT%5D%3A+", repository_link.clone()), "https://github.com/sponsors/LelouchFR".to_string(), "mailto:dev.baptiste.zahnow@gmail.com".to_string()]
                                .iter()
                                .enumerate()
                                .map(|(i, link)| {
                                    let texts: Vec<&str> = vec!["Report a Bug", "Feature Request", "Support the Project", "Contact Us"];

                                    html! {
                                        <li>
                                            <a onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)} href={link.clone()}>
                                                {texts[i]}
                                            </a>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </ul>
                </section>
            </section>
            <section class={classes!("license")}>
                <p><a href={format!("{}/blob/main/LICENSE", repository_link.clone())} onmouseover={on_mouse_hover.clone()} class={classes!(&*random_hover_color)}>{"\u{00a9} 2024 Baptiste Zahnow. Apache V2.0 License."}</a></p>
            </section>
        </footer>
    }
}
