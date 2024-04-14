
// TODO maybe move this out to a shrs_themes crate later

use serde::Deserialize;
use crossterm::style::{ContentStyle, Stylize};
use shrs::theme::Theme;

#[derive(Debug, Deserialize)]
pub enum ColorTheme {
    /// Vanilla shrs color theme
    Default,
    /// Completely white theme
    PolarBear,
    /// Completely green theme to make everyone think you are a hackerman
    Hacker,
    /// Various shades of blue
    Ocean,
}

impl ColorTheme {
    pub fn to_theme(self) -> Theme {
        match self {
            ColorTheme::Default => Theme::default(),
            ColorTheme::PolarBear => Theme {
                out_style: ContentStyle::new().white(),
                err_style: ContentStyle::new().white(),
                selection_style: ContentStyle::new().white(),
                completion_style: ContentStyle::new().white(),
                suggestion_style: ContentStyle::new().white(),
            },
            ColorTheme::Hacker => Theme {
                out_style: ContentStyle::new().green(),
                err_style: ContentStyle::new().green(),
                selection_style: ContentStyle::new().green(),
                completion_style: ContentStyle::new().green(),
                suggestion_style: ContentStyle::new().green(),
            },
            ColorTheme::Ocean => Theme {
                out_style: ContentStyle::new().blue(),
                err_style: ContentStyle::new().blue(),
                selection_style: ContentStyle::new().blue(),
                completion_style: ContentStyle::new().blue(),
                suggestion_style: ContentStyle::new().blue(),
            },
        }
    }
}


// TODO builtin for switching the colorscheme

