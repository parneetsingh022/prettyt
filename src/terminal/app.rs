//! Terminal emulator identification profiling.
//!
//! We can add terminal specific features by detecting which
//! terminal we running on.

use std::sync::OnceLock;

#[cfg(test)]
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TerminalApp {
    AppleTerminal,
    Unknown,
}

// A test-only mutable static configuration layer to hold our mock target override
#[cfg(test)]
static MOCK_APP_OVERRIDE: Mutex<Option<TerminalApp>> = Mutex::new(None);

static TERMINAL_APP: OnceLock<TerminalApp> = OnceLock::new();

fn detect_terminal_app_internal() -> TerminalApp {
    use std::env;

    if let Ok(program) = env::var("TERM_PROGRAM")
        && program == "Apple_Terminal"
    {
        return TerminalApp::AppleTerminal;
    }
    TerminalApp::Unknown
}

/// A test-only mock override that bypasses environment checks completely.
#[cfg(test)]
pub(crate) fn force_mock_terminal_app(app: Option<TerminalApp>) {
    *MOCK_APP_OVERRIDE.lock().unwrap() = app;
}

pub(crate) fn get_terminal_app() -> TerminalApp {
    // If a mock override is explicitly set in a test context, return it directly
    #[cfg(test)]
    {
        if let Some(mocked_app) = *MOCK_APP_OVERRIDE.lock().unwrap() {
            return mocked_app;
        }
    }

    *TERMINAL_APP.get_or_init(detect_terminal_app_internal)
}
