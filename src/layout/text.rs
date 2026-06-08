//! A text widget that can be displayed within the layout system.
//!
//! `Text` displays one or more lines of text and integrates with the layout
//! system for sizing and rendering.
//!
//! Text can be displayed as-is or customized with a [`Style`].
//!
//! # Examples
//!
//! ```rust
//! use prettyt::layout::Text;
//! let text = Text::new("Hello, world!");
//!
//! // `Text` implements `core::fmt::Display`, so it can be printed directly.
//! println!("{}", text);
//! ```
//!
//! Applying a style:
//!
//! ```rust
//! use prettyt::{CSSColor, make_style};
//! use prettyt::layout::Text;
//!
//! // Create a bold red-on-grey style and apply it to the text widget.
//! let style = make_style!(fg(CSSColor::RED), bg(CSSColor::GREY), bold);
//! let text = Text::new("Warning").with_style(style);
//!
//! println!("{}", text);
//! ```
//!
use core::{cmp, fmt};

use crate::Style;
use crate::layout::{LayoutDisplay, Renderable, SizeHint};
use crate::terminal::visual_line_width;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text<'a> {
    /// The text to display.
    pub text: &'a str,

    /// Optional style used when rendering.
    pub style: Option<Style>,
}

impl<'a> Text<'a> {
    /// Creates a new text widget.
    pub fn new(text: &'a str) -> Self {
        Self { text, style: None }
    }

    /// Returns this text widget with the given style applied.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<'a> fmt::Display for Text<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // temporary display configuration proxy
        let display = LayoutDisplay {
            layout: self,
            width: 80,
        };

        fmt::Display::fmt(&display, f)
    }
}

impl<'a> Renderable for Text<'a> {
    fn measure(&self, max_width: usize) -> SizeHint {
        // Find the longest single visual line to propose a correct width hint
        let mut max_visual_len = 0;
        for line in self.text.lines() {
            max_visual_len = cmp::max(max_visual_len, visual_line_width(line));
        }

        SizeHint {
            min: cmp::min(max_visual_len, max_width),
            max: Some(cmp::min(max_visual_len, max_width)),
        }
    }

    fn total_rows(&self, _width: usize) -> usize {
        self.text.lines().count()
    }

    fn row_width(&self, row_idx: usize, _width: usize) -> usize {
        // If your text component includes prettyt ANSI sequences,
        // you would use your `visible_width()` stripping loop here.
        self.text
            .lines()
            .nth(row_idx)
            .map(visual_line_width)
            .unwrap_or(0)
    }

    fn render_row(&self, row_idx: usize, _width: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(line) = self.text.lines().nth(row_idx) {
            match self.style {
                Some(style) => write!(f, "{}", style.apply(line))?,
                None => f.write_str(line)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::LayoutDisplay;
    use crate::test_utils::MockTerminalGuard;
    use crate::{Color, ColorLevel, Style};

    #[test]
    fn test_single_line_text() {
        let left = format!("{}", Text::new("Single line text."));
        let right = "Single line text.";
        assert_eq!(left, right);
    }

    #[test]
    fn test_multi_line_text() {
        let text_raw = "This is line 1.\nThis is line 2.\nThis is line 3.";
        let left = format!("{}", Text::new(text_raw));
        assert_eq!(left, text_raw);
    }

    #[test]
    fn test_empty_text() {
        let txt = Text::new("");
        assert_eq!(txt.total_rows(80), 0);
        assert_eq!(txt.row_width(0, 80), 0);
        assert_eq!(txt.measure(80).min, 0);

        let left = format!("{}", txt);
        assert_eq!(left, "");
    }

    #[test]
    fn test_pure_newlines() {
        let txt = Text::new("\n\n");

        // Two blank vertical rows are allocated
        assert_eq!(txt.total_rows(80), 2);

        // Individual empty lines have 0 visual width
        assert_eq!(txt.row_width(0, 80), 0);
        assert_eq!(txt.row_width(1, 80), 0);
    }

    #[test]
    fn test_standard_emojis() {
        // Standard single-character emojis like 🦀 and 🚀 take exactly 2 terminal visual slots.
        // "Hello " (6) + "🦀" (2) + " " (1) + "🚀" (2) = 11 visual width slots.
        let txt = Text::new("Hello 🦀 🚀");
        assert_eq!(txt.row_width(0, 80), 11);

        let hint = txt.measure(80);
        assert_eq!(hint.min, 11);
    }

    #[test]
    fn test_east_asian_cjk_characters() {
        // Full-width East Asian characters like 华 (China/Splendid) take up exactly 2 visual slots.
        // "Hello " (6) + "华" (2) = 8 visual width slots.
        let txt = Text::new("Hello 华");
        assert_eq!(txt.row_width(0, 80), 8);
    }

    #[test]
    fn test_hindi_devanagari_scripts() {
        // Hindi alphabets (Devanagari script) take exactly 1 visual slot per base letter.
        // "नमस्ते" (Namaste) -> न (1) + म (1) + स (1) + त (1) + combining marks (0) = 4 visual slots.
        // "Hello " (6) + "नमस्ते" (4) = 10 visual width slots.
        let txt = Text::new("Hello नमस्ते");
        assert_eq!(txt.row_width(0, 80), 10);
    }

    #[test]
    fn test_measure_and_row_width_with_ansi_tags() {
        // ANSI style escape codes take up 0 visual width.
        // "\x1b[1mHello\x1b[0m" (5) + " " (1) + "\x1b[31mWorld!\x1b[0m" (6) = 12 visual width slots.
        let raw_styled = "\x1b[1mHello\x1b[0m \x1b[31mWorld!\x1b[0m";
        let txt = Text::new(raw_styled);

        assert_eq!(txt.total_rows(80), 1);
        assert_eq!(txt.row_width(0, 80), 12);

        let hint = txt.measure(80);
        assert_eq!(hint.min, 12);
    }

    #[test]
    fn test_measure_clamping_with_max_width() {
        let txt = Text::new("LongStringUnbroken"); // 18 chars

        // When max_width constraint is smaller than the text, it must clamp
        let hint = txt.measure(10);
        assert_eq!(hint.min, 10);
        assert_eq!(hint.max, Some(10));
    }

    #[test]
    fn test_multi_line_asymmetrical_widths() {
        let txt = Text::new("Short\nVeryLongLineHere\nTiny");

        assert_eq!(txt.total_rows(80), 3);
        assert_eq!(txt.row_width(0, 80), 5); // "Short"
        assert_eq!(txt.row_width(1, 80), 16); // "VeryLongLineHere"
        assert_eq!(txt.row_width(2, 80), 4); // "Tiny"

        // measure must reflect the LONGEST line's visual width
        let hint = txt.measure(80);
        assert_eq!(hint.min, 16);
    }

    #[test]
    fn test_out_of_bounds_graceful_handling() {
        let txt = Text::new("Line 1\nLine 2");

        // Requesting an invalid row index should safely return 0 width and not panic
        assert_eq!(txt.row_width(5, 80), 0);

        // Rendering an invalid row index should pass cleanly with nothing written
        struct Wrapper<'a>(&'a Text<'a>, usize);
        impl<'a> fmt::Display for Wrapper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.render_row(self.1, 80, f)
            }
        }

        let rendered_invalid = format!("{}", Wrapper(&txt, 5));
        assert_eq!(rendered_invalid, "");
    }

    #[test]
    fn test_text_new_defaults_to_none_style() {
        let text_element = Text::new("Hello, World!");
        assert_eq!(text_element.text, "Hello, World!");
        assert_eq!(text_element.style, None);
    }

    #[test]
    fn test_text_with_style_builder() {
        let custom_style = Style::new().fg(Color::Red).bold();
        let text_element = Text::new("Styled Content").with_style(custom_style);

        assert_eq!(text_element.style, Some(custom_style));
    }

    #[test]
    fn test_render_row_without_style() {
        let text_element = Text::new("Line 1\nLine 2");

        // Setup a dummy standard Formatter target via a custom Display wrapper
        let rendered_output = format!(
            "{}",
            LayoutDisplay {
                layout: &text_element,
                width: 80
            }
        );

        // LayoutDisplay joins rows with newlines, check if it printed perfectly plain
        assert_eq!(rendered_output, "Line 1\nLine 2");
    }

    #[test]
    fn test_render_row_with_style_applies_ansi_escapes() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);

        let custom_style = Style::new().fg(Color::Red);
        let text_element = Text::new("Alert").with_style(custom_style);

        let rendered = format!(
            "{}",
            LayoutDisplay {
                layout: &text_element,
                width: 80
            }
        );

        // Verify that prettyt wrapped the line row value and appended the trailing reset sequence (\x1b[0m)
        assert_eq!(rendered, "\x1b[31mAlert\x1b[0m");
    }

    #[test]
    fn test_row_width_ignores_ansi_escapes_via_visual_width() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);

        let custom_style = Style::new().fg(Color::Green).bold();
        let text_element = Text::new("Hello").with_style(custom_style);

        // Even though the actual streamed row text will include escape characters,
        // row_width evaluates only the underlying string slice via visual_line_width.
        let layout_width = text_element.row_width(0, 80);
        assert_eq!(layout_width, 5);
    }

    #[test]
    fn test_total_rows_and_measure_bounds() {
        let text_element = Text::new("Short\nVery Long Line Here");

        assert_eq!(text_element.total_rows(80), 2);

        let size_hint = text_element.measure(80);
        // The longest line ("Very Long Line Here") has 19 characters
        assert_eq!(size_hint.min, 19);
        assert_eq!(size_hint.max, Some(19));
    }
}
