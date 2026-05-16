pub mod color;
pub use self::color::Color;

use self::color::{Layer, to_ansi_string};

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
            prefix.push_str(&to_ansi_string(color, Layer::Foreground));
        }

        if let Some(color) = self.bg {
            prefix.push_str(&to_ansi_string(color, Layer::Background));
        }

        if prefix.is_empty() {
            value.to_string()
        } else {
            format!("{}{}\x1b[0m", prefix, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_default_style() {
        assert_eq!(
            Style::new(),
            Style {
                fg: None,
                bg: None,
                bold: false,
            }
        );
    }

    #[test]
    fn fg_sets_foreground_color() {
        let style = Style::new().fg(Color::RED);

        assert_eq!(style.fg, Some(Color::RED));
        assert_eq!(style.bg, None);
        assert!(!style.bold);
    }

    #[test]
    fn bg_sets_background_color() {
        let style = Style::new().bg(Color::BLUE);

        assert_eq!(style.fg, None);
        assert_eq!(style.bg, Some(Color::BLUE));
        assert!(!style.bold);
    }

    #[test]
    fn fg_and_bg_can_be_chained() {
        let style = Style::new().fg(Color::RED).bg(Color::BLUE);

        assert_eq!(style.fg, Some(Color::RED));
        assert_eq!(style.bg, Some(Color::BLUE));
    }

    #[test]
    fn apply_without_style_returns_plain_text() {
        let style = Style::new();

        assert_eq!(style.apply("hello"), "hello");
    }

    #[test]
    fn apply_with_foreground_wraps_text_with_ansi_reset() {
        let style = Style::new().fg(Color::RED);

        assert_eq!(
            style.apply("hello"),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string(Color::RED, Layer::Foreground)
            )
        );
    }

    #[test]
    fn apply_with_background_wraps_text_with_ansi_reset() {
        let style = Style::new().bg(Color::BLUE);

        assert_eq!(
            style.apply("hello"),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string(Color::BLUE, Layer::Background)
            )
        );
    }

    #[test]
    fn apply_with_foreground_and_background_orders_fg_before_bg() {
        let style = Style::new().fg(Color::RED).bg(Color::BLUE);

        assert_eq!(
            style.apply("hello"),
            format!(
                "{}{}hello\x1b[0m",
                to_ansi_string(Color::RED, Layer::Foreground),
                to_ansi_string(Color::BLUE, Layer::Background),
            )
        );
    }

    #[test]
    fn apply_accepts_any_display_value() {
        let style = Style::new().fg(Color::GREEN);

        assert_eq!(
            style.apply(42),
            format!(
                "{}42\x1b[0m",
                to_ansi_string(Color::GREEN, Layer::Foreground)
            )
        );
    }
}
