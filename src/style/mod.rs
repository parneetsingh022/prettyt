pub mod color;
pub mod registry;

pub use self::color::Color;

use self::color::to_ansi_string;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);

        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);

        self
    }

    pub fn apply(&self, value: impl std::fmt::Display) -> String {
        let mut prefix = String::new();

        if let Some(color) = self.fg {
            prefix.push_str(&to_ansi_string(color, false));
        }

        if let Some(color) = self.bg {
            prefix.push_str(&to_ansi_string(color, true));
        }

        if prefix.is_empty() {
            value.to_string()
        } else {
            format!("{}{}\x1b[0m", prefix, value)
        }
    }
}
