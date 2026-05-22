//! Terminal emulator identification profiling.
//!
//! We can add terminal specific features by detecting which
//! terminal we running on.

use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TerminalApp {
    AppleTerminal,
    Unknown,
}

fn detect_terminal_app_internal() -> TerminalApp {
    use std::env;

    if let Ok(program) = env::var("TERM_PROGRAM")
        && program == "Apple_Terminal"
    {
        return TerminalApp::AppleTerminal;
    }
    TerminalApp::Unknown
}

static TERMINAL_APP: OnceLock<TerminalApp> = OnceLock::new();

pub(crate) fn get_terminal_app() -> TerminalApp {
    *TERMINAL_APP.get_or_init(detect_terminal_app_internal)
}
