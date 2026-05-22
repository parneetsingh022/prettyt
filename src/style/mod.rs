//! Text manipulation properties, custom coloring models, and evaluation macros.
//!
//! This module houses the [`Style`] structure which operates as a builder container
//! for stacking properties like foreground colors, backgrounds, and font weights.

pub mod color;
pub mod macros;

pub use self::color::Color;
use self::color::{Layer, to_ansi_string, to_ansi_string_inner};
use crate::terminal::{ColorLevel, TerminalApp, get_cached_level, get_terminal_app};

/// A builder profile container storing terminal styling codes.
///
/// Modifiers chain fluently. The styles are only wrapped around text during
/// evaluation by the [`apply`](Style::apply) method.
///
/// # Environment Awareness
/// If the host system declares color limitations (e.g. `NO_COLOR` is found or stdout is
/// piped out of a TTY), the text escapes disappear entirely, outputting clean, unstyled strings.
///
/// # Examples
/// ```rust
/// use prettyt::style::{Style, Color};
///
/// let configuration = Style::new()
///     .fg(Color::Ansi256(220))
///     .bg(Color::Black)
///     .italic();
///
/// println!("{}", configuration.apply("Standardized Output"));
/// ```
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
    /// Creates a blank style state with no modifiers active.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the foreground text color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);

        self
    }

    /// Sets the background text color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);

        self
    }

    /// Appends a bold font emphasis attribute.
    pub fn bold(mut self) -> Self {
        self.bold = true;

        self
    }

    /// Appends an underline attribute.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Appends an italic attribute.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Appends a strikethrough attribute.
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Appends a faint/dim attribute.
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Appends an inversion attribute that natively swaps foreground and background colors.
    pub fn invert(mut self) -> Self {
        self.invert = true;
        self
    }

    /// Wraps the provided type with the built ANSI escape codes.
    ///
    /// Accepts any type implementing [`Display`](std::fmt::Display). If color support is absent,
    /// it falls back gracefully to a standard unstyled string copy.
    pub fn apply(&self, value: impl std::fmt::Display) -> String {
        self.apply_inner(value, true)
    }

    /// Formats and wraps a displayable value with the active style configuration attributes.
    ///
    /// * `value`: The target text payload to style.
    /// * `detect_color`: If true, respects the environment's terminal capability profile (stripping styles on headless CI or file pipes); if false, bypasses all guards to force formatting generation (critical for unit testing isolation).
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

        // Convert the input value into our working string allocation baseline
        let mut working_string = value.to_string();

        // FALLBACK: If strikethrough is requested on Apple Terminal, manually build
        // the strikethrough layout using Unicode combining character streams.
        if self.strikethrough && detect_color && get_terminal_app() == TerminalApp::AppleTerminal {
            let mut fallback_string = String::with_capacity(working_string.len() * 2);

            // help to prevent striking thorugh ANSI-code in the string
            let mut in_ansi: bool = false;

            for c in working_string.chars() {
                fallback_string.push(c);

                // Track if we are inside an ANSI escape sequence (\x1b...m)
                // so we don't inject strikethrough characters into formatting codes and corrupt them.
                if c == '\x1b' {
                    in_ansi = true
                } else if c == 'm' && in_ansi {
                    in_ansi = false;
                    continue;
                }
                // Skip control characters (like \x1b) so we don't inject strikethrough overlays into raw ANSI escape codes and corrupt them.
                if !c.is_control() && !in_ansi {
                    fallback_string.push('\u{0336}');
                }
            }
            working_string = fallback_string;
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

        // Native standard ANSI sequence addition execution
        if self.strikethrough {
            // If we didn't use the Unicode fallback engine (either because we are on a different
            // terminal or because environment capability tracking is explicitly disabled in tests),
            // safely append the standard ANSI formatting escape sequence.
            if !detect_color || get_terminal_app() != TerminalApp::AppleTerminal {
                prefix.push_str("\x1b[9m");
            }
        }

        if prefix.is_empty() {
            working_string
        } else {
            format!("{}{}\x1b[0m", prefix, working_string)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::TerminalApp;
    use crate::terminal::app::force_mock_terminal_app;
    use crate::terminal::registry::force_mock_color_level;

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
        let style = Style::new().fg(Color::Red);

        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, None);
        assert!(!style.bold);
    }

    #[test]
    fn bg_sets_background_color() {
        let style = Style::new().bg(Color::Blue);

        assert_eq!(style.fg, None);
        assert_eq!(style.bg, Some(Color::Blue));
        assert!(!style.bold);
    }

    #[test]
    fn fg_and_bg_can_be_chained() {
        let style = Style::new().fg(Color::Red).bg(Color::Blue);

        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, Some(Color::Blue));
    }

    #[test]
    fn apply_without_style_returns_plain_text() {
        let style = Style::new();

        assert_eq!(style.apply_inner("hello", false), "hello");
    }

    #[test]
    fn apply_with_foreground_wraps_text_with_ansi_reset() {
        let style = Style::new().fg(Color::Red);

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string_inner(Color::Red, Layer::Foreground)
            )
        );
    }

    #[test]
    fn apply_with_background_wraps_text_with_ansi_reset() {
        let style = Style::new().bg(Color::Blue);

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string_inner(Color::Blue, Layer::Background)
            )
        );
    }

    #[test]
    fn apply_with_foreground_and_background_orders_fg_before_bg() {
        let style = Style::new().fg(Color::Red).bg(Color::Blue);

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}{}hello\x1b[0m",
                to_ansi_string_inner(Color::Red, Layer::Foreground),
                to_ansi_string_inner(Color::Blue, Layer::Background),
            )
        );
    }

    #[test]
    fn apply_accepts_any_display_value() {
        let style = Style::new().fg(Color::Green);

        assert_eq!(
            style.apply_inner(42, false),
            format!(
                "{}42\x1b[0m",
                to_ansi_string_inner(Color::Green, Layer::Foreground)
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
        let style = Style::new().fg(Color::Red).bg(Color::Blue).bold();

        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, Some(Color::Blue));
        assert!(style.bold);
    }

    #[test]
    fn apply_with_bold_wraps_text_with_bold_ansi_and_reset() {
        let style = Style::new().bold();

        assert_eq!(style.apply_inner("hello", false), "\x1b[1mhello\x1b[0m");
    }

    #[test]
    fn apply_with_foreground_background_and_bold_orders_bold_after_colors() {
        let style = Style::new().fg(Color::Red).bg(Color::Blue).bold();

        assert_eq!(
            style.apply_inner("hello", false),
            format!(
                "{}{}\x1b[1mhello\x1b[0m",
                to_ansi_string_inner(Color::Red, Layer::Foreground),
                to_ansi_string_inner(Color::Blue, Layer::Background),
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
            .fg(Color::Red)
            .bg(Color::Blue)
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
                to_ansi_string_inner(Color::Red, Layer::Foreground),
                to_ansi_string_inner(Color::Blue, Layer::Background),
            )
        );
    }

    #[test]
    fn apply_with_strikethrough_uses_unicode_fallback_on_apple_terminal() {
        force_mock_terminal_app(Some(TerminalApp::AppleTerminal));
        force_mock_color_level(Some(ColorLevel::Ansi256));

        assert_eq!(get_terminal_app(), TerminalApp::AppleTerminal);
        assert_eq!(get_cached_level(), ColorLevel::Ansi256);

        // Force the system to think it's running on Apple Terminal
        // Note: In a real test environment, you would ensure your OnceLock state
        // matches or mock the underlying detection if you want to test it in pure isolation.
        let style = Style::new().strikethrough();

        // Evaluate text format mapping with detect_color set to true
        let result = style.apply_inner("abc", true);

        // it must contain the combining characters
        // e + \u{0336} instead of the ANSI escape code \x1b[9m
        assert!(result.contains('\u{0336}'));
        assert!(!result.contains("\x1b[9m"));

        force_mock_terminal_app(None);
        force_mock_color_level(None);
    }

    #[test]
    fn apply_with_strikethrough_uses_ansi_escape_on_standard_terminals() {
        // Force the system to act as an Unknown/Standard terminal
        force_mock_terminal_app(Some(TerminalApp::Unknown));
        force_mock_color_level(Some(ColorLevel::Ansi256));

        assert_eq!(get_terminal_app(), TerminalApp::Unknown);
        assert_eq!(get_cached_level(), ColorLevel::Ansi256);

        let style = Style::new().strikethrough();
        let result = style.apply_inner("abc", true);

        // Verify that it uses standard ANSI strings on standard platforms
        assert!(result.contains("\x1b[9m"));
        assert!(!result.contains('\u{0336}'));

        // Clear mock state
        force_mock_terminal_app(None);
        force_mock_color_level(None);
    }

    #[test]
    fn apply_with_strikethrough_preserves_nested_ansi_escape_sequences_in_apple_terminal() {
        use crate::terminal::app::{TerminalApp, force_mock_terminal_app};
        use crate::terminal::registry::force_mock_color_level;

        // Force Apple Terminal context and mock color capability to TrueColor
        force_mock_terminal_app(Some(TerminalApp::AppleTerminal));
        force_mock_color_level(Some(ColorLevel::TrueColor));

        assert_eq!(get_terminal_app(), TerminalApp::AppleTerminal);
        assert_eq!(get_cached_level(), ColorLevel::TrueColor);

        // Build an input string that already contains a standard ANSI color code sequence (\x1b[31m)
        // This emulates styling text that has already been colored by another formatting pass.
        let colored_text = "\x1b[31mtest\x1b[0m".to_string();
        let style = Style::new().strikethrough();

        // Evaluate mapping with detect_color set to false to force execution in headless environments
        let result = style.apply_inner(colored_text, true);

        // Verify that the raw formatting escape blocks remained completely clean and uncorrupted
        assert!(result.contains("\x1b[31m"));
        assert!(result.contains("\x1b[0m"));

        // Verify that the printable characters inside the sequence were successfully struck through
        // The text sequence should emerge transformed cleanly as: t + \u{0336} + e + \u{0336} ...
        assert!(result.contains("t\u{0336}e\u{0336}s\u{0336}t\u{0336}"));

        // Reset mock state to keep the parallel test runners fully isolated
        force_mock_terminal_app(None);
        force_mock_color_level(None);
    }
}
