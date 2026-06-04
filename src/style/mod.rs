//! Text manipulation properties, custom coloring models, and evaluation macros.
//!
//! This module houses the [`Style`] structure which operates as a builder container
//! for stacking properties like foreground colors, backgrounds, and font weights.

pub mod color;
pub mod macros;

use core::fmt;

pub use self::color::Color;
use self::color::{Layer, to_ansi_string};
use crate::terminal::{ColorLevel, get_cached_level};

/// A lazy, zero-allocation wrapper that binds a [`Style`] configuration to a value.
///
/// This type is returned by [`Style::apply`]. It does not perform any text processing
/// or string allocations upon creation. Instead, it implements [`fmt::Display`],
/// deferring the evaluation of ANSI escape sequences until the wrapper is explicitly
/// streamed into a formatting funnel (like `println!`, `format!`, or `write!`).
///
/// # Technical Notes
///
/// * **Zero-Allocation:** It purely holds a copied copy of the stack-allocated [`Style`]
///   and a borrowed reference to the underlying data.
/// * **Stream-Pass:** Calling `.to_string()` on this type *will* cause an allocation
///   inherent to creating a new `String`. To preserve zero-allocation performance,
///   pass this struct directly into formatting macros.
pub struct StyledRef<'a, T: fmt::Display + ?Sized> {
    style: Style,
    value: &'a T,
}

impl<'a, T: fmt::Display + ?Sized> fmt::Display for StyledRef<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.style.is_plain() || get_cached_level() == ColorLevel::None {
            return fmt::Display::fmt(self.value, f);
        }

        if let Some(color) = self.style.fg {
            to_ansi_string(f, color, Layer::Foreground)?;
        }

        if let Some(color) = self.style.bg {
            to_ansi_string(f, color, Layer::Background)?;
        }

        for attr_escape in self.style.active_attributes() {
            f.write_str(attr_escape)?;
        }

        fmt::Display::fmt(self.value, f)?;
        // Append the final ANSI reset sequence
        f.write_str("\x1b[0m")?;

        Ok(())
    }
}

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

    // this function checks and returns true if no style is applied to the text.
    pub(crate) fn is_plain(&self) -> bool {
        // Both fg and bg should have a value and they also can't be Color::None.
        // This check is necessary because if user tries Color::None with both fg and bg
        // the code will append ending ansi character `\x1b[0m` otherwise.
        let is_fg = self.fg.is_some() && (self.fg != Some(Color::None));
        let is_bg = self.bg.is_some() && (self.bg != Some(Color::None));

        !(is_fg
            || is_bg
            || self.bold
            || self.underline
            || self.italic
            || self.strikethrough
            || self.dim
            || self.invert)
    }

    /// Returns a list of active basic ANSI text attribute sequence strings.
    pub(crate) fn active_attributes(&self) -> impl Iterator<Item = &'static str> {
        [
            (self.bold, "\x1b[1m"),
            (self.dim, "\x1b[2m"),
            (self.italic, "\x1b[3m"),
            (self.underline, "\x1b[4m"),
            (self.invert, "\x1b[7m"),
            (self.strikethrough, "\x1b[9m"),
        ]
        .into_iter()
        .filter_map(|(active, escape)| active.then_some(escape))
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

    /// Applies the style configuration to a value that implements [`fmt::Display`].
    ///
    /// Rather than executing string formatting immediately or allocating memory on the heap,
    /// this method copies the `Style` settings and captures a reference to the given value
    /// inside a lazy [`StyledRef`] wrapper.
    ///
    /// The actual ANSI escape sequence evaluation and text rendering are deferred entirely
    /// until the returned wrapper is processed by a formatting macro (like `println!` or `write!`),
    /// ensuring a completely zero-allocation operation.
    ///
    /// # Lifetimes
    ///
    /// * `'a`: Bound exclusively to the lifetime of the input `value`. Because `Style` implements
    ///   `Copy`, its lifecycle is decoupled from the return type, allowing the originating style
    ///   instance to be immediately reused.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prettyt::{Style, Color};
    ///
    /// let highlight = Style::new().fg(Color::Cyan).bold();
    ///
    /// // The same style instance can be reused simultaneously to format different values
    /// let phrase_one = highlight.apply("Hello");
    /// let phrase_two = highlight.apply(&42);
    ///
    /// println!("{} World! The answer is {}.", phrase_one, phrase_two);
    /// ```
    pub fn apply<'a, T: fmt::Display + ?Sized>(&self, value: &'a T) -> StyledRef<'a, T> {
        StyledRef {
            style: *self,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::color::to_ansi_string_for_test;
    use crate::test_utils::MockTerminalGuard;

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
        // Acquires the global `TEST_MUTEX` to prevent concurrent test threads from racing on `CACHED_LEVEL`.
        // Passing `None` clears any override and returns it to `__Uninitialized` state.
        let _guard = MockTerminalGuard::acquire(None);
        let style = Style::new();

        assert_eq!(format!("{}", style.apply("hello")), "hello");
    }

    #[test]
    fn apply_with_foreground_wraps_text_with_ansi_reset() {
        // Force the test context to simulate a terminal capable of rendering colors
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);

        let style = Style::new().fg(Color::Red);

        // 2. Use format!() to trigger the Display stream implementation
        assert_eq!(
            format!("{}", style.apply("hello")),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string_for_test(Color::Red, Layer::Foreground)
            )
        );
    }

    // ################# These tests check if our downsampling engine works properly ##################
    #[test]
    fn downsampling_truecolor_remains_unchanged() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);

        // Vivid Coral/Orange RGB
        let style = Style::new().fg(Color::Rgb(255, 128, 0));
        let result = format!("{}", style.apply("test"));

        // Should preserve direct 24-bit RGB ANSI escapes sequence (\x1b[38;2;R;G;Bm)
        assert_eq!(result, "\x1b[38;2;255;128;0mtest\x1b[0m");
    }

    #[test]
    fn downsampling_rgb_to_ansi256_palette() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::Ansi256);

        // Pure RGB Red matches index 196 in the 256-color palette index spectrum
        let style = Style::new().fg(Color::Rgb(255, 0, 0));
        let result = format!("{}", style.apply("test"));

        // Should convert to a fixed 256-color escape sequence (\x1b[38;5;Indexm)
        assert_eq!(result, "\x1b[38;5;196mtest\x1b[0m");
    }

    #[test]
    fn downsampling_rgb_to_basic_ansi16_bucket() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::Basic);

        // Pure RGB Red downsamples to a standard 16-color Red variant
        let style = Style::new().fg(Color::Rgb(255, 0, 0));
        let result = format!("{}", style.apply("test"));

        // Should convert to a standard basic foreground sequence (\x1b[31m)
        assert_eq!(result, "\x1b[31mtest\x1b[0m");
    }

    #[test]
    fn downsampling_ansi256_to_basic_ansi16_bucket() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::Basic);

        // Index 196 represents a variant of Red which collapses back to standard Red (31)
        let style = Style::new().fg(Color::Ansi256(196));
        let result = format!("{}", style.apply("test"));

        assert_eq!(result, "\x1b[31mtest\x1b[0m");
    }

    #[test]
    fn downsampling_strips_all_formatting_when_level_is_none() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::None);

        // Build a highly decorated style configuration
        let style = Style::new()
            .fg(Color::Rgb(100, 200, 255))
            .bg(Color::Ansi256(45))
            .bold()
            .underline()
            .italic();

        let result = format!("{}", style.apply("hello"));

        // Everything should fall back cleanly to a pure unstyled string copy
        assert_eq!(result, "hello");
    }

    #[test]
    fn downsampling_keeps_basic_attributes_even_if_colors_are_none() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::Basic);

        // Color::None means color output is skipped, but structural properties like bold stay
        let style = Style::new().fg(Color::None).bold();
        let result = format!("{}", style.apply("text"));

        assert_eq!(result, "\x1b[1mtext\x1b[0m");
    }

    #[test]
    fn downsampling_handles_background_conversions_uniformly() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::Basic);

        // Verify background calculations match the foreground downsampling pipeline shifts
        let style = Style::new().bg(Color::Rgb(0, 0, 255)); // Pure Blue
        let result = format!("{}", style.apply("bg"));

        // Should fall back gracefully to a basic background blue escape string (\x1b[44m)
        assert_eq!(result, "\x1b[44mbg\x1b[0m");
    }

    // ####################################

    #[test]
    fn apply_with_background_wraps_text_with_ansi_reset() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);

        let style = Style::new().bg(Color::Blue);

        assert_eq!(
            format!("{}", style.apply("hello")),
            format!(
                "{}hello\x1b[0m",
                to_ansi_string_for_test(Color::Blue, Layer::Background)
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
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);
        let style = Style::new().bold();

        assert_eq!(format!("{}", style.apply("hello")), "\x1b[1mhello\x1b[0m");
    }
    #[test]
    fn apply_with_foreground_background_and_bold_orders_bold_after_colors() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);
        let style = Style::new().fg(Color::Red).bg(Color::Blue).bold();

        assert_eq!(
            format!("{}", style.apply("hello")),
            format!(
                "{}{}\x1b[1mhello\x1b[0m",
                to_ansi_string_for_test(Color::Red, Layer::Foreground),
                to_ansi_string_for_test(Color::Blue, Layer::Background),
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
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);
        let style = Style::new()
            .dim()
            .italic()
            .underline()
            .invert()
            .strikethrough();

        assert_eq!(
            format!("{}", style.apply("hello")),
            "\x1b[2m\x1b[3m\x1b[4m\x1b[7m\x1b[9mhello\x1b[0m"
        );
    }

    #[test]
    fn all_styles_can_be_chained_with_fg_and_bg() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);
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
            format!("{}", style.apply("hello")),
            format!(
                "{}{}\x1b[1m\x1b[2m\x1b[3m\x1b[4m\x1b[7m\x1b[9mhello\x1b[0m",
                to_ansi_string_for_test(Color::Red, Layer::Foreground),
                to_ansi_string_for_test(Color::Blue, Layer::Background),
            )
        );
    }

    #[test]
    fn apply_with_strikethrough_uses_ansi_escape_on_standard_terminals() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::Ansi256);
        assert_eq!(get_cached_level(), ColorLevel::Ansi256);

        let style = Style::new().strikethrough();
        let result = format!("{}", style.apply("abc"));

        assert!(result.contains("\x1b[9m"));
        assert!(!result.contains('\u{0336}'));
    }

    #[test]
    fn apply_with_color_none_does_not_append_ansi_reset() {
        let _guard = MockTerminalGuard::acquire(ColorLevel::TrueColor);

        // Create a style containing explicitly Color::None
        let style = Style::new().fg(Color::None).bg(Color::None);

        // The output should be EXACTLY the input string, with NO trailing reset code (\x1b[0m)
        assert_eq!(format!("{}", style.apply("hello")), "hello");
    }
}
