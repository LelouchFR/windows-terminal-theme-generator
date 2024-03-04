use rand::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
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

// pub async fn get_json_lang<T: for<'de> Deserialize<'de>>(file: &str, lang: &str) -> T { // FIXME problem_with_the_generic_type
//     let fetched_data: T = Request::get(file).send().await.unwrap().json().await.unwrap();
//
//     let data_filtered = match lang {
//         "en" => fetched_data.en,
//         "fr" => fetched_data.fr,
//         "de" => fetched_data.de,
//         "sp" => fetched_data.sp,
//         "hi" => fetched_data.hi,
//         "kr" => fetched_data.kr,
//         "jp" => fetched_data.jp,
//         "ru" => fetched_data.ru,
//         _ => fetched_data.en,
//     };
//     data_filtered
// }

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
