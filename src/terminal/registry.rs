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

    if level != ColorLevel::Uninitialized {
        return level;
    }

    *COLOR_SUPPORT.get_or_init(detect_color_level)
}

#[cfg(test)]
mod tests {
    use super::*;

    // A small helper utility to reset the atomic cache state to Uninitialized
    // before running each isolated test block case.
    fn reset_atomic_cache() {
        CACHED_LEVEL.store(0, Ordering::Release);
    }

    #[test]
    fn test_set_override_persists_globally() {
        reset_atomic_cache();

        // Enforce an explicit override
        set_override(ColorLevel::Ansi256);

        // Verify that get_cached_level honors the active atomic override bypass
        assert_eq!(get_cached_level(), ColorLevel::Ansi256);

        // Change the override state on the fly
        set_override(ColorLevel::None);
        assert_eq!(get_cached_level(), ColorLevel::None);

        // Cleanup state
        clear_override();
    }

    #[test]
    fn test_clear_override_restores_fallback_detection() {
        reset_atomic_cache();

        // Set a temporary override rule
        set_override(ColorLevel::TrueColor);
        assert_eq!(get_cached_level(), ColorLevel::TrueColor);

        // Clear the override—the cache state falls back to 0 (Uninitialized)
        clear_override();

        let current_atomic = u8_to_color_level(CACHED_LEVEL.load(Ordering::Relaxed));
        assert_eq!(current_atomic, ColorLevel::Uninitialized);
    }

    #[test]
    fn test_u8_and_color_level_bijective_mapping() {
        // Ensure serialization math matches perfectly across the boundary spectrum
        let levels = [
            ColorLevel::Uninitialized,
            ColorLevel::None,
            ColorLevel::Basic,
            ColorLevel::Ansi256,
            ColorLevel::TrueColor,
        ];

        for level in levels {
            let serialized = color_level_to_u8(level);
            let deserialized = u8_to_color_level(serialized);
            assert_eq!(level, deserialized);
        }
    }
}
