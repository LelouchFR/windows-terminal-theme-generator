use stylist::{css, StyleSource};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowsTerminalTheme {
    pub black: String,
    #[serde(rename(serialize = "brightBlack"))]
    pub bright_black: String,
    pub red: String,
    #[serde(rename(serialize = "brightRed"))]
    pub bright_red: String,
    pub green: String,
    #[serde(rename(serialize = "brightGreen"))]
    pub bright_green: String,
    pub yellow: String,
    #[serde(rename(serialize = "brightYellow"))]
    pub bright_yellow: String,
    pub blue: String,
    #[serde(rename(serialize = "brightBlue"))]
    pub bright_blue: String,
    pub purple: String,
    #[serde(rename(serialize = "brightPurple"))]
    pub bright_purple: String,
    pub cyan: String,
    #[serde(rename(serialize = "brightCyan"))]
    pub bright_cyan: String,
    pub white: String,
    #[serde(rename(serialize = "brightWhite"))]
    pub bright_white: String,
    pub name: String,
    #[serde(rename(serialize = "cursorColor"))]
    pub cursor_color: String,
    pub background: String,
    pub foreground: String,
    #[serde(rename(serialize = "selectionBackground"))]
    pub selection_background: String,
}

impl WindowsTerminalTheme {
    pub fn new(black: String, bright_black: String, red: String, bright_red: String, green: String, bright_green: String, yellow: String, bright_yellow: String, blue: String, bright_blue: String, purple: String, bright_purple: String, cyan: String, bright_cyan: String, white: String, bright_white: String, name: String, cursor_color: String, background: String, foreground: String, selection_background: String) -> WindowsTerminalTheme {
        WindowsTerminalTheme {
            black,
            bright_black,
            red,
            bright_red,
            green,
            bright_green,
            yellow,
            bright_yellow,
            blue,
            bright_blue,
            purple,
            bright_purple,
            cyan,
            bright_cyan,
            white,
            bright_white,
            name,
            cursor_color,
            background,
            foreground,
            selection_background,
        }
    }

    pub fn get_css_vars(&self) -> StyleSource {
        css!(
            --black: ${&self.black};
            --bright-black: ${&self.bright_black};
            --red: ${&self.red};
            --bright-red: ${&self.bright_red};
            --green: ${&self.green};
            --bright-green: ${&self.bright_green};
            --yellow: ${&self.yellow};
            --bright-yellow: ${&self.bright_yellow};
            --blue: ${&self.blue};
            --bright-blue: ${&self.bright_blue};
            --purple: ${&self.purple};
            --bright-purple: ${&self.bright_purple};
            --cyan: ${&self.cyan};
            --bright-cyan: ${&self.bright_cyan};
            --white: ${&self.white};
            --bright-white: ${&self.bright_white};
            --background: ${&self.background};
            --foreground: ${&self.foreground};
            --selection-background: ${&self.selection_background};
            --cursor-color: ${&self.cursor_color};
        )
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            self.black.clone(),
            self.bright_black.clone(),
            self.red.clone(),
            self.bright_red.clone(),
            self.green.clone(),
            self.bright_green.clone(),
            self.yellow.clone(),
            self.bright_yellow.clone(),
            self.blue.clone(),
            self.bright_blue.clone(),
            self.purple.clone(),
            self.bright_purple.clone(),
            self.cyan.clone(),
            self.bright_cyan.clone(),
            self.white.clone(),
            self.bright_white.clone(),
            self.background.clone(),
            self.foreground.clone(),
            self.selection_background.clone(),
            self.cursor_color.clone(),
        ]
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json_str) => json_str,
            Err(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone)]
pub struct HSL {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

impl HSL {
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> HSL {
        let clamped_hue = hue % 360.0;
        let clamped_saturation = saturation.max(0.0).min(100.0);
        let clamped_lightness = lightness.max(0.0).min(100.0);

        HSL {
            hue: clamped_hue,
            saturation: clamped_saturation,
            lightness: clamped_lightness,
        }
    }

    pub fn to_negative(&self) -> HSL {
        self.to_rgb().get_complementary().to_hsl()
    }

    #[allow(dead_code)]
    pub fn get_complementary(&self) -> HSL {
        HSL::new(self.hue + 180.0, self.saturation, self.lightness)
    }

    #[allow(dead_code)]
    pub fn get_adjacent(&self, phi: f64) -> Vec<HSL> {
        let hue_1: HSL = HSL::new(self.hue + phi, self.saturation, self.lightness);
        let hue_2: HSL = HSL::new(self.hue - phi, self.saturation, self.lightness);
        
        vec![hue_1, hue_2]
    }

    #[allow(dead_code)]
    pub fn get_triad(&self, phi: f64) -> Vec<HSL> {
        let hue_complimentary: HSL = self.get_complementary();
        let hue_1: HSL = HSL::new(hue_complimentary.hue + phi, self.saturation, self.lightness);
        let hue_2: HSL = HSL::new(hue_complimentary.hue - phi, self.saturation, self.lightness);

        vec![hue_complimentary, hue_1, hue_2]
    }

    fn hue_to_rgb(&self, start: f64, mid: f64, end: f64) -> f64 {
        let t = match end {
            end if end < 0.0 => end + 1.0,
            end if end > 1.0 => end - 1.0,
            _ => end,
        };

        match (t < 1.0 / 6.0, t < 1.0 / 2.0, t < 2.0 / 3.0) {
            (true, _, _) => start + (mid - start) * 6.0 * t,
            (false, true, _) => mid,
            (false, false, true) => start + (mid - start) * (2.0 / 3.0 - t) * 6.0,
            (false, false, false) => start,
        }
    }

    pub fn to_rgb(&self) -> RGB {
        let (hue, saturation, lightness) = (self.hue / 360.0, self.saturation / 100.0, self.lightness / 100.0);

        let q = match lightness < 0.5 {
            true => lightness * (1.0 + saturation),
            false => lightness + saturation - lightness * saturation,
        };

        let p = 2.0 * lightness - q;

        let red = self.hue_to_rgb(p, q, hue + 1.0 / 3.0);
        let green = self.hue_to_rgb(p, q, hue);
        let blue = self.hue_to_rgb(p, q, hue - 1.0 / 3.0);

        RGB::new(red * 255.0, green * 255.0, blue * 255.0)
    }
    
    pub fn to_hex(&self) -> String {
        self.to_rgb().to_hex()
    }
}

impl RGB {
    pub fn new(red: f64, green: f64, blue: f64) -> RGB {
        let clamped_red = red.max(0.0).min(255.0) as u8;
        let clamped_green = green.max(0.0).min(255.0) as u8;
        let clamped_blue = blue.max(0.0).min(255.0) as u8;

        RGB {
            red: clamped_red,
            green: clamped_green,
            blue: clamped_blue,
        }
    }

    pub fn get_complementary(&self) -> RGB {
        RGB::new(255.0 - self.red as f64, 255.0 - self.green as f64, 255.0 - self.blue as f64)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    pub fn to_hsl(&self) -> HSL {
        let (red, green, blue) = (self.red as f64 / 255.0, self.green as f64 / 255.0, self.blue as f64 / 255.0);

        let min_value = red.min(green).min(blue);
        let max_value = red.max(green).max(blue);
        let delta = max_value - min_value;
        let (mut hue, mut saturation, mut lightness);
        
        hue = match (delta == 0.0, max_value == red, max_value == green) {
            (true, _, _) => 0.0,
            (false, true, _) => ((green - blue) / delta) % 6.0,
            (false, false, true) => (blue - red) / delta + 2.0,
            (false, false, false) => (red - green) / delta + 4.0,
        };

        hue *= 60.0;

        if hue < 0.0 {
            hue += 360.0;
        }

        lightness = (max_value + min_value) / 2.0;
        saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        saturation *= 100.0;
        lightness *= 100.0;

        HSL::new(hue, saturation, lightness)
    }
}

pub trait ToHSL {
    fn to_hsl(&self) -> Option<HSL>;
}

impl ToHSL for String {
    fn to_hsl(&self) -> Option<HSL> {
        let hex_str = if self.starts_with("#") {
            &self[1..]
        } else {
            self
        };

        if hex_str.len() != 6 {
            return None;
        }

        let red = u8::from_str_radix(&hex_str[0..2], 16).ok()? as f64;
        let green = u8::from_str_radix(&hex_str[2..4], 16).ok()? as f64;
        let blue = u8::from_str_radix(&hex_str[4..6], 16).ok()? as f64;

        Some(RGB::new(red, green, blue).to_hsl())
    }
}
