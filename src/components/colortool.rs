use crate::utils::make_chars;
use yew::prelude::*;

fn mapped_text(
    background_colors: Vec<String>,
    foreground_colors: Vec<String>,
    text: &str,
    vertical_text: Vec<String>,
) -> Html {
    foreground_colors
        .iter()
        .enumerate()
        .map(|(i, fg_color)| {
            let v_text = if i % 2 == 0 {
                format!(
                    "{}{}",
                    vertical_text[i % 8].clone(),
                    make_chars(' ', 5)
                )
            } else {
                format!(
                    "1;{}{}",
                    vertical_text[(i - 1) % 8].clone(),
                    make_chars(' ', 3)
                )
            };
        
            html! {
                <>
                    <span class={classes!("white")}>{v_text}</span>
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
        })
        .collect::<Html>()
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
        "bright-white",
    ];

    let back_color_classes: Vec<&str> = vec![
        "black", "red", "green", "yellow", "blue", "purple", "cyan", "white",
    ];

    let background_color_classes: Vec<String> = back_color_classes
        .iter()
        .map(|color| format!("{}--as-background", color))
        .collect();
    
    let vertical_text: Vec<String> = (30..=37).map(|x| format!("{}m", x)).collect();
    let horizontal_text: Vec<String> = (40..=47).map(|x| format!("{}m", x)).collect();
    let text: &str = "gYw";

    let header: Html = html! {
        <p class={classes!(color_classes[14])}>
            {(0..=9).map(|_| " ").collect::<String>()}
            {
                horizontal_text
                    .iter()
                    .map(|bg_color| format!("{}{}", bg_color, " ".repeat(5)))
                    .collect::<String>()
            }
            <br />
        </p>
    };
    
    html! {
        <section class={classes!("colortool")}>
            <pre>
                {header}
                {
                    mapped_text(
                        background_color_classes.clone(),
                        color_classes.iter().map(|x| format!("{}", x)).collect(),
                        text,
                        vertical_text
                    )
                }
            </pre>
        </section>
    }
}
