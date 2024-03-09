use yew::prelude::*;
use rand::prelude::*;
use crate::colors::colors::{HSL, WindowsTerminalTheme, ToHSL};

pub fn generate_theme(is_white_mode: bool) -> WindowsTerminalTheme {
    let mut rng = rand::thread_rng(); // generates a random number generator to get a number from 0 to 1
    
    let lightnesses: Vec<f64> = vec![
        rng.gen::<f64>() * 30.0 + 20.0,
        rng.gen::<f64>() * 30.0 + 50.0
    ];
    
    // (initial color is going to be a shade of black and the brightBlack)
    let mut colors: Vec<(String, String)> = vec![
        (
            HSL::new(0.0, 0.0, rng.gen::<f64>() * 10.0).to_hex(),
            HSL::new(0.0, 0.0, rng.gen::<f64>() * 35.0 + 10.0).to_hex()
        )
    ];
    
    for i in 1..=6 {
        colors.push(
            (
                HSL::new(rng.gen::<f64>() * ((360.0 / 6.0) * i as f64) + 15.0, 100.0, lightnesses[0]).to_hex(),
                HSL::new(rng.gen::<f64>() * ((360.0 / 6.0) * i as f64) + 15.0, 100.0, lightnesses[1]).to_hex()
            )
        );
    }
    
    colors.push(
        (HSL::new(0.0, 0.0, 100.0).to_hex(), HSL::new(0.0, 0.0, 95.0).to_hex())
    );

    // 16 colors (8 pairs of normal colors and bright colors)
    let indexes_of_sorted_colors: Vec<usize> = vec![0, 6, 2, 1, 4, 5, 3, 7];
    let sorted_colors: Vec<(String, String)> =  indexes_of_sorted_colors.iter().map(|&i| colors[i].clone()).collect();

    // 1 theme name
    let theme_name: String = "Rust Generated Theme".to_string();

    // 4 colors (cursorColor, background, foreground, selectionBackground)
    let suplementary_colors: Vec<HSL> = vec![
        HSL::new(0.0, 0.0, 100.0),
        HSL::new(0.0, 0.0, 10.0),
        HSL::new(0.0, 0.0, 95.0),
        HSL::new(0.0, 0.0, 95.0)
    ];

    let mut new_generated_theme = vec![];
    for (color, bright_color) in sorted_colors {
        new_generated_theme.push(color);
        new_generated_theme.push(bright_color);
    }

    new_generated_theme.push(theme_name.clone());

    for suplements in suplementary_colors {
        new_generated_theme.push(suplements.to_hex());
    }
    
    let impl_new_theme: WindowsTerminalTheme = match is_white_mode {
        true => WindowsTerminalTheme::new(
            new_generated_theme[0].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[1].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[2].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[3].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[4].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[5].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[6].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[7].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[8].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[9].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[10].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[11].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[12].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[13].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[14].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[15].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[16].clone(),
            new_generated_theme[17].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[18].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[19].clone().to_hsl().unwrap().to_negative().to_hex(),
            new_generated_theme[20].clone().to_hsl().unwrap().to_negative().to_hex(),
        ),
        false => WindowsTerminalTheme::new(
            new_generated_theme[0].clone(),
            new_generated_theme[1].clone(),
            new_generated_theme[2].clone(),
            new_generated_theme[3].clone(),
            new_generated_theme[4].clone(),
            new_generated_theme[5].clone(),
            new_generated_theme[6].clone(),
            new_generated_theme[7].clone(),
            new_generated_theme[8].clone(),
            new_generated_theme[9].clone(),
            new_generated_theme[10].clone(),
            new_generated_theme[11].clone(),
            new_generated_theme[12].clone(),
            new_generated_theme[13].clone(),
            new_generated_theme[14].clone(),
            new_generated_theme[15].clone(),
            new_generated_theme[16].clone(),
            new_generated_theme[17].clone(),
            new_generated_theme[18].clone(),
            new_generated_theme[19].clone(),
            new_generated_theme[20].clone()
        ),
    };
    
    impl_new_theme
}

#[allow(non_upper_case_globals)]
pub const color_classes: [&str;20] = [
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
    "background",
    "foreground",
    "selection-background",
    "cursor-color"
];

#[derive(Properties, PartialEq)]
pub struct ColoredTextHeaderProps {
    pub value: String,
    pub class: String,
}

#[function_component(ColoredTextHeader)]
pub fn colored_text_header(props: &ColoredTextHeaderProps) -> Html {

    html! {
        <h1 class={classes!(&props.class)}>
            {
                props.value
                    .split(",")
                    .into_iter()
                    .enumerate()
                    .map(|(i, word)| html!{<span class={classes!(color_classes[i * 2 + 3])}>{word}</span>})
                    .collect::<Html>()
            }
        </h1>
    }
}

#[function_component(WindowsTerminalText)]
pub fn windows_terminal_text() -> Html {
    html! {
        <p>
            <span class={classes!(color_classes[15])}>{"Windows Terminal"}</span><br />
            <span class={classes!(color_classes[7])}>
                <img src="/svg/register_path_style.svg" alt={"C:\\"} width={"64px"} draggable={"false"}/>
                {" git"}
            </span>
            <span class={classes!(color_classes[15])}>{" diff"}</span>
            <span class={classes!(color_classes[1])}>{" -w"}</span><br />
            <span class={classes!(color_classes[15])}>{"diff --git a/win b/win"}</span><br />
            <span class={classes!(color_classes[13])}>{"@@ -1 +1 @@"}</span><br />
            <span class={classes!(color_classes[3])}>{"-   Windows Console"}</span><br />
            <span class={classes!(color_classes[5])}>{"+   Windows Terminal!"}</span><br />
            <span class={classes!(color_classes[7])}>
                <img src="/svg/register_path_style.svg" alt={"C:\\"} width={"64px"} draggable={"false"}/>
                {" Write-Host"}
            </span>
            <span class={classes!(color_classes[13])}>{" \"✏!\""}</span>
        </p>
    }
}

pub fn get_random_color_on_hover() -> String {
    let mut rng = rand::thread_rng();
    color_classes[rng.gen_range(0..=color_classes.len())].to_string()
}
