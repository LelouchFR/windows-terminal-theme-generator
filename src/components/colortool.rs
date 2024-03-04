use yew::prelude::*;
use gloo_console::log;
use yew::virtual_dom::VNode;

fn mapped_text(background_colors: Vec<&str>, foreground_colors: Vec<&str>, text: &str) -> Html {
    (background_colors.iter().zip(foreground_colors))
        .map(|(bg_color, fg_color)| {
            log!(format!("{}, {}", bg_color, fg_color));
            html! {<><span class={classes!(bg_color.to_string(), fg_color.to_string())}>{"  "}{text}{"  "}</span><span>{" "}</span></>}
        })
        .collect::<Html>()
}

#[function_component(ColorTool)]
pub fn color_tool() -> Html {
    let foreground_colors_classes: Vec<&str> = vec!["black", "red", "green", "yellow", "blue", "purple", "cyan", "white"];
    let foreground_colors_classes_2: Vec<&str> = vec!["bright-black", "bright-red", "bright-green", "bright-yellow", "bright-blue", "bright-purple", "bright-cyan", "bright-white"];
    let background_colors_classes: Vec<&str> = vec!["black--as-background", "red--as-background", "green--as-background", "yellow--as-background", "blue--as-background", "purple--as-background", "cyan--as-background", "white--as-background"];
    let foreground_colors: Vec<&str> = vec!["30m", "31m", "32m", "33m", "34m", "35m", "36m", "37m"];
    let background_colors: Vec<&str> = vec!["40m", "41m", "42m", "43m", "44m", "45m", "46m", "47m"];
    let text: &str = "gYw";

    let header: Html = html! {
        <pre>
            {(0..=9).map(|_| " ").collect::<String>()}
            {
                background_colors
                    .iter()
                    .map(|bg_color| format!("{}{}", bg_color, " ".repeat(5)))
                    .collect::<String>()
            }
        </pre>
    };

    let color_lines: Vec<Html> = foreground_colors.iter().map(|fg_color| {
        html! {
            <pre>
                {
                    format!("{}{}", fg_color, " ".repeat(5))
                }
                {mapped_text(background_colors_classes.clone(), foreground_colors_classes.clone(), text)}
            </pre>
        }
    }).collect();

    let bright_color_lines: Vec<Html> = foreground_colors.iter().map(|fg_color| {
        let fg_bright_color = format!("1;{}", fg_color);
        html! {
            <pre>
                {
                    format!("{}{}", fg_bright_color, " ".repeat(3))
                }
                {mapped_text(background_colors_classes.clone(), foreground_colors_classes_2.clone(), text)}
            </pre>
        }
    }).collect();

    let lines = color_lines.iter().zip(bright_color_lines.iter()).map(|(color, bright_color)| {
        html! {
            <>
                {<VNode as Clone>::clone(&*color)}
                {<VNode as Clone>::clone(&*bright_color)}
            </>
        }
    }).collect::<Html>();
    
    html! {
        <div>
            {header}
            {lines}
        </div>
    }
}
