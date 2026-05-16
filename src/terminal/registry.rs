use std::sync::OnceLock;

use crate::terminal::{ColorLevel, detect_color_level};

static COLOR_SUPPORT: OnceLock<ColorLevel> = OnceLock::new();

/// Returns the detected terminal color support level, caching it after the first call.
///
/// Uses a thread-safe, lazy initialization to check environment variables and TTY status
/// once per program execution.
pub(crate) fn get_cached_level() -> ColorLevel {
    *COLOR_SUPPORT.get_or_init(detect_color_level)
}
