pub mod color;
pub use self::color::Color;

use self::color::{Layer, to_ansi_string, to_ansi_string_inner};

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
        self.apply_inner(value, true)
    }

    pub(crate) fn apply_inner(&self, value: impl std::fmt::Display, rollback: bool) -> String {
        let mut prefix = String::new();

        // Github tests do not have terminal/environment detection available, so use the
        // inner ANSI formatter directly instead of the environment-aware wrapper.
        let ansi_fn = if rollback {
            to_ansi_string
        } else {
            to_ansi_string_inner
        };

        if let Some(color) = self.fg {
            prefix.push_str(&ansi_fn(color, Layer::Foreground));
        }

        if let Some(color) = self.bg {
            prefix.push_str(&ansi_fn(color, Layer::Background));
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

        assert_eq!(style.apply_inner("hello", false), "hello");
    }

    #[test]
    fn apply_with_foreground_wraps_text_with_ansi_reset() {
        let style = Style::new().fg(Color::RED);

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string_inner(Color::RED, Layer::Foreground)
            )
        );
    }

    #[test]
    fn apply_with_background_wraps_text_with_ansi_reset() {
        let style = Style::new().bg(Color::BLUE);

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string_inner(Color::BLUE, Layer::Background)
            )
        );
    }

    #[test]
    fn apply_with_foreground_and_background_orders_fg_before_bg() {
        let style = Style::new().fg(Color::RED).bg(Color::BLUE);

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}{}hello\x1b[0m",
                to_ansi_string_inner(Color::RED, Layer::Foreground),
                to_ansi_string_inner(Color::BLUE, Layer::Background),
            )
        );
    }

    #[test]
    fn apply_accepts_any_display_value() {
        let style = Style::new().fg(Color::GREEN);

        assert_eq!(
            style.apply_inner(42, false),
            format!(
                "{}42\x1b[0m",
                to_ansi_string_inner(Color::GREEN, Layer::Foreground)
            )
        );
    }
}
