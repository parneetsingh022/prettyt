pub mod color;
pub use self::color::Color;

use self::color::{Layer, to_ansi_string, to_ansi_string_inner};
use crate::terminal::{ColorLevel, get_cached_level};

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    underline: bool,
    italic: bool,
    strikethrough: bool,
    dim: bool,
    invert: bool,
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

    pub fn bold(mut self) -> Self {
        self.bold = true;

        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    pub fn invert(mut self) -> Self {
        self.invert = true;
        self
    }

    pub fn apply(&self, value: impl std::fmt::Display) -> String {
        self.apply_inner(value, true)
    }

    pub(crate) fn apply_inner(&self, value: impl std::fmt::Display, detect_color: bool) -> String {
        let has_styles = self.fg.is_some()
            || self.bg.is_some()
            || self.bold
            || self.underline
            || self.italic
            || self.strikethrough
            || self.dim
            || self.invert;

        if !has_styles || detect_color && get_cached_level() == ColorLevel::None {
            return value.to_string();
        }

        let mut prefix = String::new();

        // GitHub tests do not have terminal/environment detection available, so use the
        // inner ANSI formatter directly instead of the environment-aware wrapper.
        let ansi_fn = if detect_color {
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

        if self.bold {
            prefix.push_str("\x1b[1m");
        }

        if self.dim {
            prefix.push_str("\x1b[2m");
        }
        if self.italic {
            prefix.push_str("\x1b[3m");
        }
        if self.underline {
            prefix.push_str("\x1b[4m");
        }
        if self.invert {
            prefix.push_str("\x1b[7m");
        }
        if self.strikethrough {
            prefix.push_str("\x1b[9m");
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
                underline: false,
                italic: false,
                strikethrough: false,
                dim: false,
                invert: false
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

    #[test]
    fn bold_sets_bold_to_true() {
        let style = Style::new().bold();

        assert!(style.bold);
        assert_eq!(style.fg, None);
        assert_eq!(style.bg, None);
    }

    #[test]
    fn bold_can_be_chained_with_fg_and_bg() {
        let style = Style::new().fg(Color::RED).bg(Color::BLUE).bold();

        assert_eq!(style.fg, Some(Color::RED));
        assert_eq!(style.bg, Some(Color::BLUE));
        assert!(style.bold);
    }

    #[test]
    fn apply_with_bold_wraps_text_with_bold_ansi_and_reset() {
        let style = Style::new().bold();

        assert_eq!(style.apply_inner("hello", false), "\x1b[1mhello\x1b[0m");
    }

    #[test]
    fn apply_with_foreground_background_and_bold_orders_bold_after_colors() {
        let style = Style::new().fg(Color::RED).bg(Color::BLUE).bold();

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}{}\x1b[1mhello\x1b[0m",
                to_ansi_string_inner(Color::RED, Layer::Foreground),
                to_ansi_string_inner(Color::BLUE, Layer::Background),
            )
        );
    }

    #[test]
    fn underline_sets_underline_to_true() {
        let style = Style::new().underline();

        assert!(style.underline);
    }

    #[test]
    fn italic_sets_italic_to_true() {
        let style = Style::new().italic();

        assert!(style.italic);
    }

    #[test]
    fn strikethrough_sets_strikethrough_to_true() {
        let style = Style::new().strikethrough();

        assert!(style.strikethrough);
    }

    #[test]
    fn dim_sets_dim_to_true() {
        let style = Style::new().dim();

        assert!(style.dim);
    }

    #[test]
    fn invert_sets_invert_to_true() {
        let style = Style::new().invert();

        assert!(style.invert);
    }

    #[test]
    fn apply_with_text_styles_wraps_text_with_ansi_and_reset() {
        let style = Style::new()
            .dim()
            .italic()
            .underline()
            .invert()
            .strikethrough();

        assert_eq!(
            style.apply_inner("hello", false),
            "\x1b[2m\x1b[3m\x1b[4m\x1b[7m\x1b[9mhello\x1b[0m"
        );
    }

    #[test]
    fn all_styles_can_be_chained_with_fg_and_bg() {
        let style = Style::new()
            .fg(Color::RED)
            .bg(Color::BLUE)
            .bold()
            .dim()
            .italic()
            .underline()
            .invert()
            .strikethrough();

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}{}\x1b[1m\x1b[2m\x1b[3m\x1b[4m\x1b[7m\x1b[9mhello\x1b[0m",
                to_ansi_string_inner(Color::RED, Layer::Foreground),
                to_ansi_string_inner(Color::BLUE, Layer::Background),
            )
        );
    }
}
