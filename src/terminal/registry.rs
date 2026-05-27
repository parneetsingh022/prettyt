use crate::terminal::{ColorLevel, detect_color_level};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

#[cfg(test)]
use std::sync::Mutex;

// Atomic state values:
// 0 => Uninitialized (must detect)
// 1 => None
// 2 => Basic
// 3 => Ansi256
// 4 => TrueColor
static CACHED_LEVEL: AtomicU8 = AtomicU8::new(0);

fn u8_to_color_level(level: u8) -> ColorLevel {
    match level {
        0 => ColorLevel::Uninitialized,
        1 => ColorLevel::None,
        2 => ColorLevel::Basic,
        3 => ColorLevel::Ansi256,
        4 => ColorLevel::TrueColor,
        _ => unreachable!(),
    }
}

fn color_level_to_u8(level: ColorLevel) -> u8 {
    match level {
        ColorLevel::Uninitialized => 0,
        ColorLevel::None => 1,
        ColorLevel::Basic => 2,
        ColorLevel::Ansi256 => 3,
        ColorLevel::TrueColor => 4,
    }
}

#[cfg(test)]
static MOCK_COLOR_LEVEL: Mutex<Option<ColorLevel>> = Mutex::new(None);

static COLOR_SUPPORT: OnceLock<ColorLevel> = OnceLock::new();

#[cfg(test)]
pub(crate) fn force_mock_color_level(level: Option<ColorLevel>) {
    *MOCK_COLOR_LEVEL.lock().unwrap() = level;
}

/// Forcefully override the global color level at runtime,
/// bypassing any automatic environment cascades or cached values.
pub fn set_override(level: ColorLevel) {
    let state = color_level_to_u8(level);
    CACHED_LEVEL.store(state, Ordering::Release);
}

/// Clear a previously set override, resetting the engine to
/// evaluate environment variables on the next style processing block.
pub fn clear_override() {
    CACHED_LEVEL.store(0, Ordering::Release);
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
    let level = u8_to_color_level(CACHED_LEVEL.load(Ordering::Relaxed));

    if level == ColorLevel::Uninitialized {
        return level;
    }

    *COLOR_SUPPORT.get_or_init(detect_color_level)
}
