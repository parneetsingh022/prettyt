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
    Unix,
}

#[cfg(windows)]
fn get_platform() -> Platform {
    Platform::Windows
}

#[cfg(not(windows))]
fn get_platform() -> Platform {
    Platform::Unix
}

#[cfg(windows)]
/// Enables ANSI escape codes (colors/formatting) in the Windows Console.
///
/// Returns `true` if successfully enabled or already active; `false` if the
/// console handle is invalid (e.g., output is redirected) or the OS call fails.
fn enable_virtual_terminal_processing() -> bool {
    use windows_sys::Win32::System::Console::{
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE,
        SetConsoleMode,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);

        // Invalid handle / redirected output
        if handle.is_null() {
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

fn is_tty() -> bool {
    io::stdout().is_terminal()
}

/// Returns the color level
pub fn detect_color_level() -> ColorLevel {
    detect_color_level_inner(
        get_platform(),
        is_tty(),
        env::var_os("NO_COLOR").is_some(),
        env::var("FORCE_COLOR").ok().as_deref(),
        env::var("COLORTERM").ok().as_deref(),
        env::var("TERM").ok().as_deref(),
        env::var_os("WT_SESSION").is_some(),
        enable_virtual_terminal_processing(),
        env::var("TERM_PROGRAM").ok().as_deref(),
    )
}

#[allow(clippy::too_many_arguments)]
fn detect_color_level_inner(
    // Platform whose color-detection rules should be applied.
    platform: Platform,

    // Whether stdout is attached to an interactive terminal.
    is_tty: bool,
    no_color: bool,
    force_color: Option<&str>,
    colorterm: Option<&str>,
    term: Option<&str>,
    windows_terminal: bool,

    // Whether the current Windows console supports ANSI escape codes via
    // ENABLE_VIRTUAL_TERMINAL_PROCESSING.
    windows_vt_enabled: bool,

    term_program: Option<&str>,
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

    // Windows requires Virtual Terminal Processing for ANSI escapes.
    // We verify it before advertising color support.
    if platform == Platform::Windows && !windows_vt_enabled {
        return ColorLevel::None;
    }

    // TERM/COLORTERM are frequently unset on Windows, so we additionally
    // recognize Windows Terminal (WT_SESSION) as a reliable indicator of
    // TrueColor support.
    //
    // If WT_SESSION was explicitly caught (e.g. spawned inside WT natively)
    if windows_terminal {
        return ColorLevel::TrueColor;
    }

    if let Some(ct) = colorterm
        && (ct.contains("truecolor") || ct.contains("24bit"))
    {
        return ColorLevel::TrueColor;
    }

    // Explicit Apple Terminal (Terminal.app) Rule:
    // It leaves COLORTERM blank and supports 256 colors perfectly, but lacks 24-bit RGB support.
    if let Some("Apple_Terminal") = term_program {
        // Only return Ansi256 if TERM doesn't explicitly restrict it to "dumb"
        if term != Some("dumb") {
            return ColorLevel::Ansi256;
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

    // On Windows, if VT processing is available, assume TrueColor.
    // Windows terminals often do not expose COLORTERM/TERM/WT_SESSION even when
    // 24-bit color works, so a conservative fallback would under-detect many
    // modern Windows consoles.
    match platform {
        Platform::Unix => ColorLevel::Basic,
        Platform::Windows => ColorLevel::TrueColor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ColorLevel;

    #[test]
    fn returns_none_when_not_tty_without_force_color() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                false,                  // is_tty
                false,                  // no_color
                None,                   // force_color
                Some("truecolor"),      // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn no_color_disables_color_without_force_color() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                true,                   // is_tty
                true,                   // no_color
                None,                   // force_color
                Some("truecolor"),      // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn force_color_overrides_not_tty() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                false,                  // is_tty
                false,                  // no_color
                Some("3"),              // force_color
                Some("truecolor"),      // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn no_color_overrides_force_color() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                true,                   // is_tty
                true,                   // no_color
                Some("3"),              // force_color
                Some("truecolor"),      // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn force_color_zero_disables_color() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                true,                   // is_tty
                false,                  // no_color
                Some("0"),              // force_color
                Some("truecolor"),      // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn force_color_one_returns_basic() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                Some("1"),      // force_color
                None,           // colorterm
                None,           // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::Basic
        );
    }

    #[test]
    fn force_color_two_returns_ansi256() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                Some("2"),      // force_color
                None,           // colorterm
                None,           // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn force_color_three_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                Some("3"),      // force_color
                None,           // colorterm
                None,           // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn unknown_force_color_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                Some("yes"),    // force_color
                None,           // colorterm
                None,           // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn colorterm_truecolor_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,    // Platform
                true,              // is_tty
                false,             // no_color
                None,              // force_color
                Some("truecolor"), // colorterm
                None,              // term
                false,             // windows_terminal
                false,             // windows_vt_enabled
                None,              // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn colorterm_24bit_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                None,           // force_color
                Some("24bit"),  // colorterm
                None,           // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn term_dumb_returns_none() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                None,           // force_color
                None,           // colorterm
                Some("dumb"),   // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn term_256color_returns_ansi256() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                true,                   // is_tty
                false,                  // no_color
                None,                   // force_color
                None,                   // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn fallback_returns_basic() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                None,           // force_color
                None,           // colorterm
                Some("xterm"),  // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::Basic
        );
    }

    #[test]
    fn colorterm_unknown_falls_through_to_term() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,         // Platform
                true,                   // is_tty
                false,                  // no_color
                None,                   // force_color
                Some("unknown"),        // colorterm
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                false,                  // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn no_colorterm_no_term_fallback_basic() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                None,           // force_color
                None,           // colorterm
                None,           // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::Basic
        );
    }

    #[test]
    fn term_unknown_falls_back_basic() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix, // Platform
                true,           // is_tty
                false,          // no_color
                None,           // force_color
                None,           // colorterm
                Some("vt100"),  // term
                false,          // windows_terminal
                false,          // windows_vt_enabled
                None,           // term_program
            ),
            ColorLevel::Basic
        );
    }

    #[test]
    fn windows_without_vt_returns_none() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Windows, // Platform
                true,              // is_tty
                false,             // no_color
                None,              // force_color
                None,              // colorterm
                None,              // term
                false,             // windows_terminal
                false,             // windows_vt_enabled
                None,              // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn windows_with_vt_defaults_to_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Windows, // Platform
                true,              // is_tty
                false,             // no_color
                None,              // force_color
                None,              // colorterm
                None,              // term
                false,             // windows_terminal
                true,              // windows_vt_enabled
                None,              // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn windows_terminal_returns_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Windows, // Platform
                true,              // is_tty
                false,             // no_color
                None,              // force_color
                None,              // colorterm
                None,              // term
                true,              // windows_terminal
                true,              // windows_vt_enabled
                None,              // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn windows_force_color_overrides_vt_disabled() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Windows, // Platform
                true,              // is_tty
                false,             // no_color
                Some("3"),         // force_color
                None,              // colorterm
                None,              // term
                false,             // windows_terminal
                false,             // windows_vt_enabled
                None,              // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn windows_no_color_overrides_everything() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Windows,      // Platform
                true,                   // is_tty
                true,                   // no_color
                Some("3"),              // force_color
                Some("truecolor"),      // colorterm
                Some("xterm-256color"), // term
                true,                   // windows_terminal
                true,                   // windows_vt_enabled
                None,                   // term_program
            ),
            ColorLevel::None
        );
    }

    #[test]
    fn test_legacy_apple_terminal_without_colorterm_defaults_to_ansi256() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,
                true,                   // is_tty
                false,                  // no_color
                None,                   // force_color
                None,                   // colorterm (Legacy versions leave this blank)
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                true,                   // windows_vt_enabled
                Some("Apple_Terminal"), // term_program
            ),
            ColorLevel::Ansi256
        );
    }

    #[test]
    fn test_modern_apple_terminal_tahoe_with_colorterm_escalates_to_truecolor() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,
                true,                   // is_tty
                false,                  // no_color
                None,                   // force_color
                Some("truecolor"),      // colorterm (Tahoe populates this explicitly)
                Some("xterm-256color"), // term
                false,                  // windows_terminal
                true,                   // windows_vt_enabled
                Some("Apple_Terminal"), // term_program
            ),
            ColorLevel::TrueColor
        );
    }

    #[test]
    fn test_apple_terminal_honors_dumb_term_restriction() {
        assert_eq!(
            detect_color_level_inner(
                Platform::Unix,
                true,                   // is_tty
                false,                  // no_color
                None,                   // force_color
                None,                   // colorterm
                Some("dumb"),           // term explicitly restricts color
                false,                  // windows_terminal
                true,                   // windows_vt_enabled
                Some("Apple_Terminal"), // term_program
            ),
            ColorLevel::None
        );
    }
}
