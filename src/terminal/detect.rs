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
    #[doc(hidden)]
    __Uninitialized,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Platform {
    Windows,

    // Represents UNIX like platform (Unix, Linux, MacOS)
    Unix,
}

#[cfg(windows)]
fn platform() -> Platform {
    Platform::Windows
}

#[cfg(not(windows))]
fn platform() -> Platform {
    Platform::Unix
}

#[cfg(windows)]
/// Enables ANSI escape codes (colors/formatting) in the Windows Console.
///
/// Returns `true` if successfully enabled or already active; `false` if the
/// console handle is invalid (e.g., output is redirected) or the OS call fails.
fn enable_virtual_terminal_processing() -> bool {
    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        System::Console::{
            ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE,
            SetConsoleMode,
        },
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);

        // Invalid handle / redirected output
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            return false;
        }

        let mut mode = 0;

        if GetConsoleMode(handle, &mut mode) == 0 {
            return false;
        }

        if mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING != 0 {
            return true;
        }

        let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        SetConsoleMode(handle, new_mode) != 0
    }
}

#[cfg(not(windows))]
/// Stub function for non-Windows platforms (macOS/Linux).
///
/// Returns `true` automatically because Unix-like terminals support
/// ANSI escape codes out of the box.
fn enable_virtual_terminal_processing() -> bool {
    true
}

fn stdout_is_tty() -> bool {
    io::stdout().is_terminal()
}

#[cfg(windows)]
fn unix_like_terminal_on_windows() -> bool {
    let is_msys_like = env::var("MSYSTEM")
        .map(|v| {
            let v = v.to_ascii_uppercase();
            v.contains("MINGW") || v.contains("MSYS") || v.contains("CLANG")
        })
        .unwrap_or(false);

    let is_cygwin_like = env::var("TERM")
        .map(|v| v.to_ascii_lowercase().contains("cygwin"))
        .unwrap_or(false);

    let is_mintty_like = env::var_os("MINTTY_PID").is_some();

    is_msys_like || is_cygwin_like || is_mintty_like
}

#[cfg(not(windows))]
fn unix_like_terminal_on_windows() -> bool {
    false
}

#[derive(Debug)]
struct ColorDetectionInput {
    platform: Platform,
    is_tty: bool,
    no_color: bool,
    force_color: Option<String>,
    colorterm: Option<String>,
    term: Option<String>,
    windows_terminal: bool,
    windows_vt_enabled: bool,
    term_program: Option<String>,
    unix_like_on_windows: bool,
}

/// Returns the color level
pub fn detect_color_level() -> ColorLevel {
    detect_color_level_inner(ColorDetectionInput {
        platform: platform(),
        is_tty: stdout_is_tty(),
        no_color: env::var_os("NO_COLOR").is_some(),
        force_color: env::var("FORCE_COLOR").ok(),
        colorterm: env::var("COLORTERM").ok(),
        term: env::var("TERM").ok(),
        windows_terminal: env::var_os("WT_SESSION").is_some(),
        windows_vt_enabled: enable_virtual_terminal_processing(),
        term_program: env::var("TERM_PROGRAM").ok(),
        unix_like_on_windows: unix_like_terminal_on_windows(),
    })
}

fn detect_color_level_inner(input: ColorDetectionInput) -> ColorLevel {
    if input.no_color {
        return ColorLevel::None;
    }

    if let Some(force) = input.force_color.as_deref() {
        return parse_force_color(force);
    }

    if !input.is_tty {
        return ColorLevel::None;
    }

    if input.platform == Platform::Windows
        && !input.windows_vt_enabled
        && !input.unix_like_on_windows
    {
        return ColorLevel::None;
    }

    if input.windows_terminal {
        return ColorLevel::TrueColor;
    }

    if let Some(colorterm) = input.colorterm.as_deref() {
        let colorterm = colorterm.to_ascii_lowercase();

        if colorterm.contains("truecolor") || colorterm.contains("24bit") {
            return ColorLevel::TrueColor;
        }
    }

    // Explicit Apple Terminal (Terminal.app) Rule:
    // It leaves COLORTERM blank and supports 256 colors perfectly, but lacks 24-bit RGB support.
    if input.term_program.as_deref() == Some("Apple_Terminal")
        && input.term.as_deref() != Some("dumb")
    {
        return ColorLevel::Ansi256;
    }

    if let Some(term) = input.term.as_deref() {
        let term = term.to_ascii_lowercase();

        if term == "dumb" {
            return ColorLevel::None;
        }

        if term.contains("truecolor") || term.contains("24bit") {
            return ColorLevel::TrueColor;
        }

        if term.contains("256color") {
            return ColorLevel::Ansi256;
        }
    }

    // On Windows, if VT processing is available, assume TrueColor.
    // Windows terminals often do not expose COLORTERM/TERM/WT_SESSION even when
    // 24-bit color works, so a conservative fallback would under-detect many
    // modern Windows consoles.
    match input.platform {
        Platform::Unix => ColorLevel::Basic,
        Platform::Windows => ColorLevel::TrueColor,
    }
}

fn parse_force_color(value: &str) -> ColorLevel {
    match value.trim().to_ascii_lowercase().as_str() {
        "0" | "false" | "no" | "off" => ColorLevel::None,
        "" | "1" | "true" | "yes" | "on" => ColorLevel::Basic,
        "2" => ColorLevel::Ansi256,
        "3" => ColorLevel::TrueColor,
        _ => ColorLevel::TrueColor,
    }
}

#[test]
fn parse_force_color_disables_color_values() {
    for value in ["0", "false", "no", "off"] {
        assert_eq!(parse_force_color(value), ColorLevel::None, "value={value}");
    }
}

#[test]
fn parse_force_color_basic_values() {
    for value in ["", "1", "true", "yes", "on"] {
        assert_eq!(parse_force_color(value), ColorLevel::Basic, "value={value}");
    }
}

#[test]
fn parse_force_color_two_returns_ansi256() {
    assert_eq!(parse_force_color("2"), ColorLevel::Ansi256);
}

#[test]
fn parse_force_color_three_returns_truecolor() {
    assert_eq!(parse_force_color("3"), ColorLevel::TrueColor);
}

#[test]
fn parse_force_color_unknown_values_default_to_truecolor() {
    for value in ["4", "always", "maybe", "random", "256", "truecolor"] {
        assert_eq!(
            parse_force_color(value),
            ColorLevel::TrueColor,
            "value={value}"
        );
    }
}

#[test]
fn parse_force_color_is_case_insensitive() {
    for value in ["FALSE", "False", "NO", "Off", "TRUE", "Yes", "ON"] {
        let expected = match value.to_ascii_lowercase().as_str() {
            "false" | "no" | "off" => ColorLevel::None,
            "true" | "yes" | "on" => ColorLevel::Basic,
            _ => unreachable!(),
        };

        assert_eq!(parse_force_color(value), expected, "value={value}");
    }
}

#[test]
fn parse_force_color_trims_whitespace() {
    let cases = [
        (" 0 ", ColorLevel::None),
        ("\tfalse\n", ColorLevel::None),
        (" no ", ColorLevel::None),
        (" off ", ColorLevel::None),
        ("   ", ColorLevel::Basic),
        (" 1 ", ColorLevel::Basic),
        ("\ttrue\n", ColorLevel::Basic),
        (" yes ", ColorLevel::Basic),
        (" on ", ColorLevel::Basic),
        (" 2 ", ColorLevel::Ansi256),
        (" 3 ", ColorLevel::TrueColor),
        (" random ", ColorLevel::TrueColor),
    ];

    for (value, expected) in cases {
        assert_eq!(parse_force_color(value), expected, "value={value:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ColorLevel;

    #[cfg(test)]
    fn input() -> ColorDetectionInput {
        ColorDetectionInput {
            platform: Platform::Unix,
            is_tty: true,
            no_color: false,
            force_color: None,
            colorterm: None,
            term: None,
            windows_terminal: false,
            windows_vt_enabled: false,
            term_program: None,
            unix_like_on_windows: false,
        }
    }

    #[test]
    fn returns_none_when_not_tty_without_force_color() {
        let mut input = input();
        input.is_tty = false;
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_disables_color_without_force_color() {
        let mut input = input();
        input.no_color = true;
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_overrides_not_tty() {
        let mut input = input();
        input.is_tty = false;
        input.force_color = Some("3".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn no_color_overrides_force_color() {
        let mut input = input();
        input.no_color = true;
        input.force_color = Some("3".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_zero_disables_color() {
        let mut input = input();
        input.force_color = Some("0".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_one_returns_basic() {
        let mut input = input();
        input.force_color = Some("1".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn force_color_two_returns_ansi256() {
        let mut input = input();
        input.force_color = Some("2".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn force_color_three_returns_truecolor() {
        let mut input = input();
        input.force_color = Some("3".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn unknown_force_color_returns_truecolor() {
        let mut input = input();
        input.force_color = Some("unknown".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn colorterm_truecolor_returns_truecolor() {
        let mut input = input();
        input.colorterm = Some("truecolor".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn colorterm_24bit_returns_truecolor() {
        let mut input = input();
        input.colorterm = Some("24bit".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn term_dumb_returns_none() {
        let mut input = input();
        input.term = Some("dumb".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn term_256color_returns_ansi256() {
        let mut input = input();
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn fallback_returns_basic() {
        let mut input = input();
        input.term = Some("xterm".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn colorterm_unknown_falls_through_to_term() {
        let mut input = input();
        input.colorterm = Some("unknown".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn no_colorterm_no_term_fallback_basic() {
        let input = input();

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn term_unknown_falls_back_basic() {
        let mut input = input();
        input.term = Some("vt100".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn windows_without_vt_returns_none() {
        let mut input = input();
        input.platform = Platform::Windows;

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn windows_with_vt_defaults_to_truecolor() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = true;

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn windows_terminal_returns_truecolor() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_terminal = true;
        input.windows_vt_enabled = true;

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn windows_force_color_overrides_vt_disabled() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.force_color = Some("3".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn windows_no_color_overrides_everything() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.no_color = true;
        input.force_color = Some("3".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());
        input.windows_terminal = true;
        input.windows_vt_enabled = true;

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn test_legacy_apple_terminal_without_colorterm_defaults_to_ansi256() {
        let mut input = input();
        input.term = Some("xterm-256color".to_string());
        input.windows_vt_enabled = true;
        input.term_program = Some("Apple_Terminal".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn test_modern_apple_terminal_tahoe_with_colorterm_escalates_to_truecolor() {
        let mut input = input();
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());
        input.windows_vt_enabled = true;
        input.term_program = Some("Apple_Terminal".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn test_apple_terminal_honors_dumb_term_restriction() {
        let mut input = input();
        input.term = Some("dumb".to_string());
        input.windows_vt_enabled = true;
        input.term_program = Some("Apple_Terminal".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_none_wins_on_unix_even_with_truecolor_env() {
        let mut input = input();
        input.force_color = Some("0".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_basic_wins_on_unix_even_with_truecolor_env() {
        let mut input = input();
        input.force_color = Some("1".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn force_color_ansi256_wins_on_unix_even_with_truecolor_env() {
        let mut input = input();
        input.force_color = Some("2".to_string());
        input.colorterm = Some("truecolor".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn force_color_truecolor_wins_on_unix_even_with_dumb_term() {
        let mut input = input();
        input.force_color = Some("3".to_string());
        input.term = Some("dumb".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn force_color_overrides_not_tty_on_unix() {
        let mut input = input();
        input.is_tty = false;
        input.force_color = Some("2".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn force_color_false_disables_color_on_windows_terminal() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_terminal = true;
        input.windows_vt_enabled = true;
        input.force_color = Some("false".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_on_enables_basic_on_windows_without_vt() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = false;
        input.force_color = Some("on".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn force_color_two_enables_ansi256_on_windows_without_vt() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = false;
        input.force_color = Some("2".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn force_color_three_enables_truecolor_on_windows_without_vt() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = false;
        input.force_color = Some("3".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn force_color_unknown_defaults_truecolor_on_windows_without_vt() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = false;
        input.force_color = Some("always".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn no_color_still_overrides_force_color_on_windows() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.no_color = true;
        input.force_color = Some("3".to_string());
        input.windows_terminal = true;
        input.windows_vt_enabled = true;

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_overrides_not_tty_on_windows() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.is_tty = false;
        input.force_color = Some("3".to_string());
        input.windows_vt_enabled = false;

        assert_eq!(detect_color_level_inner(input), ColorLevel::TrueColor);
    }

    #[test]
    fn force_color_works_on_unix_like_windows_shell() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.unix_like_on_windows = true;
        input.windows_vt_enabled = false;
        input.force_color = Some("2".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn force_color_false_disables_unix_like_windows_shell() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.unix_like_on_windows = true;
        input.windows_vt_enabled = true;
        input.term = Some("xterm-256color".to_string());
        input.force_color = Some("off".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn force_color_overrides_colorterm_on_unix_like_windows_shell() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.unix_like_on_windows = true;
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());
        input.force_color = Some("1".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn force_color_empty_string_means_basic_even_on_windows_without_vt() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = false;
        input.force_color = Some("".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Basic);
    }

    #[test]
    fn force_color_whitespace_is_trimmed_in_detection() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.windows_vt_enabled = false;
        input.force_color = Some(" 2 ".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::Ansi256);
    }

    #[test]
    fn no_color_disables_color_on_unix_with_truecolor_env() {
        let mut input = input();
        input.no_color = true;
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_disables_color_on_unix_even_with_force_color() {
        let mut input = input();
        input.no_color = true;
        input.force_color = Some("3".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_disables_color_on_windows_terminal() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.no_color = true;
        input.windows_terminal = true;
        input.windows_vt_enabled = true;

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_disables_color_on_windows_even_with_force_color() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.no_color = true;
        input.force_color = Some("3".to_string());
        input.windows_terminal = true;
        input.windows_vt_enabled = true;

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_disables_color_on_windows_without_vt() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.no_color = true;
        input.windows_vt_enabled = false;

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_disables_color_on_unix_like_windows_shell() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.unix_like_on_windows = true;
        input.no_color = true;
        input.term = Some("xterm-256color".to_string());
        input.colorterm = Some("truecolor".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_overrides_force_color_on_unix_like_windows_shell() {
        let mut input = input();
        input.platform = Platform::Windows;
        input.unix_like_on_windows = true;
        input.no_color = true;
        input.force_color = Some("2".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_overrides_dumb_term_behavior() {
        let mut input = input();
        input.no_color = true;
        input.term = Some("dumb".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_overrides_apple_terminal_truecolor() {
        let mut input = input();
        input.no_color = true;
        input.term_program = Some("Apple_Terminal".to_string());
        input.colorterm = Some("truecolor".to_string());
        input.term = Some("xterm-256color".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }

    #[test]
    fn no_color_overrides_not_tty() {
        let mut input = input();
        input.no_color = true;
        input.is_tty = false;
        input.force_color = Some("3".to_string());

        assert_eq!(detect_color_level_inner(input), ColorLevel::None);
    }
}
