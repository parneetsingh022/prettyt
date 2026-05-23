use std::sync::OnceLock;

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
static MOCK_COLOR_LEVEL: Mutex<Option<ColorLevel>> = Mutex::new(None);

use crate::terminal::{ColorLevel, detect_color_level};

static COLOR_SUPPORT: OnceLock<ColorLevel> = OnceLock::new();

#[cfg(test)]
pub(crate) fn force_mock_color_level(level: Option<ColorLevel>) {
    *MOCK_COLOR_LEVEL.lock().unwrap() = level;
}

/// Returns the detected terminal color support level, caching it after the first call.
///
/// Uses a thread-safe, lazy initialization to check environment variables and TTY status
/// once per program execution.
pub(crate) fn get_cached_level() -> ColorLevel {
    #[cfg(test)]
    {
        if let Some(level) = *MOCK_COLOR_LEVEL.lock().unwrap() {
            return level;
        }
    }

    *COLOR_SUPPORT.get_or_init(detect_color_level)
}
