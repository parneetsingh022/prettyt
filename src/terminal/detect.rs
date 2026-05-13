//! Terminal color capability detection.
//!
//! Inspects environment variables and whether stdout is a TTY to determine
//! how many colors the terminal supports, returning a [`ColorLevel`] used
//! throughout `prettyt` to render styled output.
//!
//! ## Detection order
//! 
//! - **`FORCE_COLOR`** — if set (and not `"0"`), forces true color regardless of other signals.
//! - **TTY check** — if stdout is piped, color is disabled entirely.
//! - **`NO_COLOR`** — if set to any value, disables color ([no-color.org](https://no-color.org)).
//! - **`COLORTERM`** — `"truecolor"` or `"24bit"` advertises full RGB support.
//! - **`TERM`** — `"dumb"` means no color, `"*256color*"` means 256 colors, else basic 16.

use std::{env, io::{self, IsTerminal}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorLevel {
    None,
    Basic,     // 16 colors
    Ansi256,   // 256 colors
    TrueColor, // 24-bit
}


fn is_tty() -> bool {
    io::stdout().is_terminal()
}

pub fn detect_color_level() -> ColorLevel {
    detect_color_level_inner(
        is_tty(),
        env::var("NO_COLOR").is_ok(),
        env::var("FORCE_COLOR").ok().as_deref(),
        env::var("COLORTERM").ok().as_deref(),
        env::var("TERM").ok().as_deref(),
    )
}

fn detect_color_level_inner(
    is_tty : bool,
    no_color: bool,
    force_color: Option<&str>,
    colorterm: Option<&str>,
    term: Option<&str>,
) -> ColorLevel {

    match force_color {
        Some("0") => return ColorLevel::None,
        Some("1") => return ColorLevel::Basic,
        Some("2") => return ColorLevel::Ansi256,
        Some(_) => return ColorLevel::TrueColor,
        None => {}
    }

    if !is_tty || no_color {
        return ColorLevel::None
    }
    

    if let Some(ct) = colorterm {
        if ct.contains("truecolor") || ct.contains("24bit") {
            return ColorLevel::TrueColor;
        }
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
            detect_color_level_inner(false, false, None, Some("truecolor"), Some("xterm-256color")),
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
            detect_color_level_inner(false, false, Some("3"), Some("truecolor"), Some("xterm-256color")),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn force_color_overrides_no_color() {
        assert_eq!(
            detect_color_level_inner(true, true, Some("3"), Some("truecolor"), Some("xterm-256color")),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn force_color_zero_disables_color() {
        assert_eq!(
            detect_color_level_inner(true, false, Some("0"), Some("truecolor"), Some("xterm-256color")),
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
            detect_color_level_inner(
                true,
                false,
                None,
                Some("ansi"),
                Some("xterm-256color")
            ),
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