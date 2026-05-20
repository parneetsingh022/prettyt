//! Terminal color capability detection and environment routing.
//!
//! This module handles the core environmental discovery cascade for `prettyt`.
//! It dynamically inspects active environment variables alongside the host's standard
//! output stream configuration to negotiate an accurate [`ColorLevel`]. This safeguards downstream formatting and layout engines from polluting logs or breaking basic displays.
//!
//! # The Detection Cascade
//!
//! Resolution operates sequentially using a strict short-circuiting cascade. The first rule to match completely dictates the resulting terminal capability level:
//!
//! | Priority | Mechanism / Variable | Matching Rule & Behavior | Target Level |
//! | :---: | :--- | :--- | :--- |
//! | **1** | `FORCE_COLOR` | Matches `"0"` | [`ColorLevel::None`] |
//! | | | Matches `"1"` | [`ColorLevel::Basic`] |
//! | | | Matches `"2"` | [`ColorLevel::Ansi256`] |
//! | | | Matches any other value (e.g., `"3"`, `"yes"`) | [`ColorLevel::TrueColor`] |
//! | **2** | TTY Check (`stdout`) | Stream is piped or redirected (`!is_terminal()`) | [`ColorLevel::None`] |
//! | **3** | `NO_COLOR` | Variable is present in environment (any value) | [`ColorLevel::None`] |
//! | **4** | `COLORTERM` | Contains substring `"truecolor"` or `"24bit"` | [`ColorLevel::TrueColor`] |
//! | **5** | `TERM` | Value equals `"dumb"` | [`ColorLevel::None`] |
//! | | | Contains substring `"256color"` | [`ColorLevel::Ansi256`] |
//! | **6** | *Fallback* | Default catch-all rule for standard TTY contexts | [`ColorLevel::Basic`] |
//!
//! ---
//!
//! # Behavioral Rules & Context
//!
//! ### 1. Color Explicit Overrides (`FORCE_COLOR`)
//! The `FORCE_COLOR` environment variable acts as a master override lever. When configured, **all interactive TTY evaluations and standard `NO_COLOR` provisions are ignored entirely**. This is uniquely beneficial for continuous integration (CI) platforms, build runners, or test pipelines where styled streams need to be preserved inside automated logs.
//!
//! ### 2. Pipeline Safeguarding (TTY Detection)
//! When standard output (`stdout`) is actively piped to a subsequent utility (such as `grep` or `cat`) or dumped directly into a log file, the terminal framework flags the channel as a non-interactive text stream. `prettyt` dynamically drops styling markers out of the layout under these circumstances to ensure raw outputs remain clean and text logs are not corrupted with raw escape sequence characters.
//!
//! ### 3. Accessibility Compliance (`NO_COLOR`)
//! Consistently enforces community standard profiles defined by [no-color.org](https://no-color.org). If a user exports `NO_COLOR` inside their operating shell config, `prettyt` strips layout rendering back to raw values to respect local visibility, contrast, and screen-reader software demands.
//!
//! ### 4. Modern TrueColor Layouts (`COLORTERM` & `TERM`)
//! Advanced display managers (e.g., Alacritty, iTerm2, VS Code integrated consoles) populate `COLORTERM` to indicate support for comprehensive 24-bit RGB processing channels. When discovered, `prettyt` handles colors with pinpoint fidelity. For standard legacy devices containing `256color` in their environment strings, high-performance math vectors automatically downsample TrueColor values down into the closest indexing slot within the terminal's 256-color cube.

use std::{
    env,
    io::{self, IsTerminal},
};

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
    is_tty: bool,
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
    fn force_color_overrides_no_color() {
        assert_eq!(
            detect_color_level_inner(
                true,
                true,
                Some("3"),
                Some("truecolor"),
                Some("xterm-256color")
            ),
            ColorLevel::TrueColor
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
