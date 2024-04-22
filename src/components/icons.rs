#[derive(Debug, Clone)]
pub enum Icons {
    Random,
    Previous,
    Next,
    Theme,
    Copy,
    Share,
    Warning,
    Error,
    Validation,
    Info,
}

#[derive(Debug, Clone)]
pub struct Icon {
    pub icon: String,
}

impl Icon {
    pub fn new(icon: Icons) -> Self {
        Icon {
            icon: match icon {
                Icons::Random => "/svg/random.svg".to_string(),
                Icons::Previous => "/svg/previous.svg".to_string(),
                Icons::Next => "/svg/next.svg".to_string(),
                Icons::Theme => "/svg/moon.svg".to_string(),
                Icons::Copy => "/svg/copy.svg".to_string(),
                Icons::Share => "/svg/share.svg".to_string(),
                Icons::Warning => "/svg/warning.svg".to_string(),
                Icons::Error => "/svg/error.svg".to_string(),
                Icons::Validation => "/svg/validation.svg".to_string(),
                Icons::Info => "/svg/info.svg".to_string(),
            },
        }
    }
}
