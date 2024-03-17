use yew::prelude::*;
use crate::utils::make_chars;

fn mapped_text(background_colors: Vec<String>, foreground_colors: Vec<String>, text: &str) -> Html {
    foreground_colors.iter().map(|fg_color| {
        html! {
            <>
                {make_chars(' ', 8)}
                {
                    background_colors.iter().map(|bg_color| {
                        html! {
                            <>
                                <span class={classes!(bg_color, fg_color)}>
                                    {format!("  {}  ", text)}
                                </span>
                                <span>{" "}</span>
                            </>
                        }
                    }).collect::<Html>()
                }
                <br />
            </>
        }
    }).collect::<Html>()
}

#[function_component(ColorTool)]
pub fn color_tool() -> Html {
    let color_classes: Vec<&str> = vec![
        "black",
        "bright-black",
        "red",
        "bright-red",
        "green",
        "bright-green",
        "yellow",
        "bright-yellow",
        "blue",
        "bright-blue",
        "purple",
        "bright-purple",
        "cyan",
        "bright-cyan",
        "white",
        "bright-white"
    ];

    let back_color_classes: Vec<&str> = vec![
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "purple",
        "cyan",
        "white",
    ];

    let background_color_classes: Vec<String> = back_color_classes
        .iter()
        .map(|color| format!("{}--as-background", color))
        .collect();
    
    // FIXME need to add vertical text
    let _vertical_text: Vec<String> = (30..=37).map(|x| format!("{}m", x)).collect();
    let horizontal_text: Vec<String> = (40..=47).map(|x| format!("{}m", x)).collect();
    let text: &str = "gYw";

    let header: Html = html! {
        <>
            {(0..=9).map(|_| " ").collect::<String>()}
            {
                horizontal_text
                    .iter()
                    .map(|bg_color| format!("{}{}", bg_color, " ".repeat(5)))
                    .collect::<String>()
            }
            <br />
        </>
    };
    
    html! {
        <section class={classes!("colortool")}>
            <pre>
                {header}
                {mapped_text(background_color_classes.clone(), color_classes.iter().map(|x| format!("{}", x)).collect(), text)}
            </pre>
        </section>
    }
}
