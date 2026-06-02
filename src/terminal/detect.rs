//! Terminal color capability detection.
//!
//! Detects terminal color support by inspecting environment variables and
//! whether stdout is attached to a TTY, returning a [`ColorLevel`] used by
//! `prettyt` for styled output rendering.
//!
//! ## Detection precedence
//!
//! Detection follows this order:
//!
//! - **`NO_COLOR`** — if present, color output is fully disabled
//!   ([no-color.org](https://no-color.org)).
//! - **`FORCE_COLOR`** — if set (and not `"0"`), color output is forced
//!   even when stdout is not a TTY.
//! - **TTY check** — if stdout is not a terminal, color is disabled.
//! - **`COLORTERM`** — `"truecolor"` or `"24bit"` enables 24-bit RGB color.
//! - **`TERM`** — `"dumb"` disables color, `"*256color*"` enables 256-color
//!   support, otherwise basic ANSI colors are assumed.

use std::{
    env,
    io::{self, IsTerminal},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Describes the level of terminal color support.
pub enum ColorLevel {
    /// No color support.
    ///
    /// Output should not contain ANSI color escape sequences.
    None,

    /// Basic ANSI color support.
    ///
    /// Supports the standard 16 terminal colors.
    Basic,

    /// Extended ANSI color support.
    ///
    /// Supports 256-color ANSI escape sequences.
    Ansi256,

    /// True color support.
    ///
    /// Supports 24-bit RGB color, usually written as `16_777_216` possible colors.
    TrueColor,

    /// Internal sentinel value used before color support has been detected.
    ///
    /// > **Warning**
    /// >
    /// > This variant is for internal use only.
    /// > Downstream code should not construct it or rely on matching it.
    #[doc(alias = "Uninitialized")]
    __Uninitialized,
}

fn is_tty() -> bool {
    io::stdout().is_terminal()
}

/// Returns the color level
pub fn detect_color_level() -> ColorLevel {
    detect_color_level_inner(
        is_tty(),
        env::var_os("NO_COLOR").is_some(),
        env::var("FORCE_COLOR").ok().as_deref(),
        env::var("COLORTERM").ok().as_deref(),
        env::var("TERM").ok().as_deref(),
    )
}

fn detect_color_level_inner(
    is_tty: bool,
    no_color: bool,
    force_color: Option<&str>,
    colorterm: Option<&str>,
    term: Option<&str>,
) -> ColorLevel {
    if no_color {
        return ColorLevel::None;
    }

    match force_color {
        Some("0") => return ColorLevel::None,
        Some("1") => return ColorLevel::Basic,
        Some("2") => return ColorLevel::Ansi256,
        Some(_) => return ColorLevel::TrueColor,
        None => {}
    }

    if !is_tty {
        return ColorLevel::None;
    }

    if let Some(ct) = colorterm
        && (ct.contains("truecolor") || ct.contains("24bit"))
    {
        return ColorLevel::TrueColor;
    }

    if let Some(term) = term {
        if term == "dumb" {
            return ColorLevel::None;
        }

        if term.contains("256color") {
            return ColorLevel::Ansi256;
        }
    }

    // default fallback for TTYS
    ColorLevel::Basic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn returns_none_when_not_tty_without_force_color() {
        assert_eq!(
            detect_color_level_inner(
                false,
                false,
                None,
                Some("truecolor"),
                Some("xterm-256color")
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn no_color_disables_color_without_force_color() {
        assert_eq!(
            detect_color_level_inner(true, true, None, Some("truecolor"), Some("xterm-256color")),
            ColorLevel::None
        );
    }

    #[test]
    fn force_color_overrides_not_tty() {
        assert_eq!(
            detect_color_level_inner(
                false,
                false,
                Some("3"),
                Some("truecolor"),
                Some("xterm-256color")
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn no_color_overrides_force_color() {
        assert_eq!(
            detect_color_level_inner(
                true,
                true,
                Some("3"),
                Some("truecolor"),
                Some("xterm-256color")
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn force_color_zero_disables_color() {
        assert_eq!(
            detect_color_level_inner(
                true,
                false,
                Some("0"),
                Some("truecolor"),
                Some("xterm-256color")
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn force_color_one_returns_basic() {
        assert_eq!(
            detect_color_level_inner(true, false, Some("1"), None, None),
            ColorLevel::Basic
        );
    }

    #[test]
    fn force_color_two_returns_ansi256() {
        assert_eq!(
            detect_color_level_inner(true, false, Some("2"), None, None),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn force_color_three_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(true, false, Some("3"), None, None),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn unknown_force_color_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(true, false, Some("yes"), None, None),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn colorterm_truecolor_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(true, false, None, Some("truecolor"), None),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn colorterm_24bit_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(true, false, None, Some("24bit"), None),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn term_dumb_returns_none() {
        assert_eq!(
            detect_color_level_inner(true, false, None, None, Some("dumb")),
            ColorLevel::None
        );
    }

    #[test]
    fn term_256color_returns_ansi256() {
        assert_eq!(
            detect_color_level_inner(true, false, None, None, Some("xterm-256color")),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn fallback_returns_basic() {
        assert_eq!(
            detect_color_level_inner(true, false, None, None, Some("xterm")),
            ColorLevel::Basic
        );
    }

    #[test]
    fn colorterm_unknown_falls_through_to_term() {
        assert_eq!(
            detect_color_level_inner(true, false, None, Some("ansi"), Some("xterm-256color")),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn no_colorterm_no_term_fallback_basic() {
        assert_eq!(
            detect_color_level_inner(true, false, None, None, None),
            ColorLevel::Basic
        );
    }

    #[test]
    fn term_unknown_falls_back_basic() {
        assert_eq!(
            detect_color_level_inner(true, false, None, None, Some("vt100")),
            ColorLevel::Basic
        );
    }
}
