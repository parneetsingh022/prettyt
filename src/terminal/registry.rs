use crate::terminal::{ColorLevel, detect_color_level};
use std::sync::atomic::{AtomicU8, Ordering};

/// The global atomic cache tracking the active terminal color capability level.
///
/// This single primitive manages both the lazy initialization pass and dynamic
/// runtime overrides (`set_override`).
///
/// # Atomic State Mapping Values:
/// * `0` => __Uninitialized (Forces an evaluation of environment cascades on the next call)
/// * `1` => Evaluated and cached as `ColorLevel::None`
/// * `2` => Evaluated and cached as `ColorLevel::Basic`
/// * `3` => Evaluated and cached as `ColorLevel::Ansi256`
/// * `4` => Evaluated and cached as `ColorLevel::TrueColor`
static CACHED_LEVEL: AtomicU8 = AtomicU8::new(0);

#[cfg(test)]
pub(crate) fn reset_cached_level() {
    CACHED_LEVEL.store(0, Ordering::Release);
}

fn u8_to_color_level(level: u8) -> ColorLevel {
    match level {
        0 => ColorLevel::__Uninitialized,
        1 => ColorLevel::None,
        2 => ColorLevel::Basic,
        3 => ColorLevel::Ansi256,
        4 => ColorLevel::TrueColor,
        _ => unreachable!(),
    }
}

fn color_level_to_u8(level: ColorLevel) -> u8 {
    match level {
        ColorLevel::__Uninitialized => 0,
        ColorLevel::None => 1,
        ColorLevel::Basic => 2,
        ColorLevel::Ansi256 => 3,
        ColorLevel::TrueColor => 4,
    }
}

/// Clear a previously set override, restoring automatic (cached) detection.
///
/// Note: terminal capability detection is still cached for the life of the process
/// once it has been initialized.
pub(crate) fn get_cached_level() -> ColorLevel {
    let raw = CACHED_LEVEL.load(Ordering::Acquire);

    // if not Uninitialized
    if raw != 0 {
        return u8_to_color_level(raw);
    }

    let detected = detect_color_level();
    let new_raw = color_level_to_u8(detected);

    match CACHED_LEVEL.compare_exchange(0, new_raw, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => detected,
        Err(actual_raw) => u8_to_color_level(actual_raw),
    }
}

/// Forcefully override the global color level at runtime,
/// bypassing any automatic environment cascades or cached values.
pub fn set_override(level: ColorLevel) {
    assert!(
        level != ColorLevel::__Uninitialized,
        "ColorLevel::Uninitialized cannot be used as a color override; use clear_override() instead"
    );

    let state = color_level_to_u8(level);
    CACHED_LEVEL.store(state, Ordering::Release);
}

/// Clears any active override and invalidates the cached color level.
///
/// This resets `CACHED_LEVEL` to `0` (`ColorLevel::__Uninitialized`).
/// The next call to `get_cached_level()` will run terminal color detection again
/// and store the newly detected level in the cache.
pub fn clear_override() {
    CACHED_LEVEL.store(0, Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockTerminalGuard;

    #[test]
    fn test_set_override_persists_globally() {
        // Acquires the global `TEST_MUTEX` to prevent concurrent test threads from racing on `CACHED_LEVEL`.
        // Passing `None` clears the mock override out of the way so our real production cell gets tested.
        let _guard = MockTerminalGuard::acquire(None);

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
        // Acquires the global `TEST_MUTEX` to prevent concurrent test threads from racing on `CACHED_LEVEL`.
        // Passing `None` clears the mock override out of the way so our real production cell gets tested.
        let _guard = MockTerminalGuard::acquire(None);

        // Set a temporary override rule
        set_override(ColorLevel::TrueColor);
        assert_eq!(get_cached_level(), ColorLevel::TrueColor);

        // Clear the override—the cache state falls back to 0 (Uninitialized)
        clear_override();

        let current_atomic = u8_to_color_level(CACHED_LEVEL.load(Ordering::Relaxed));
        assert_eq!(current_atomic, ColorLevel::__Uninitialized);
    }

    #[test]
    fn test_u8_and_color_level_bijective_mapping() {
        // Ensure serialization math matches perfectly across the boundary spectrum
        let levels = [
            ColorLevel::__Uninitialized,
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

    #[test]
    fn test_set_override_rejects_uninitialized() {
        // Acquires the global `TEST_MUTEX` to prevent concurrent test threads from racing on `CACHED_LEVEL`.
        // Passing `None` clears the mock override out of the way so our real production cell gets tested.
        let _guard = MockTerminalGuard::acquire(None);

        let result = std::panic::catch_unwind(|| {
            set_override(ColorLevel::__Uninitialized);
        });

        assert!(result.is_err());

        let current_atomic = u8_to_color_level(CACHED_LEVEL.load(Ordering::Acquire));
        assert_eq!(current_atomic, ColorLevel::__Uninitialized);
    }

    #[test]
    fn test_set_override_uninitialized_does_not_clear_existing_override() {
        // Acquires the global `TEST_MUTEX` to prevent concurrent test threads from racing on `CACHED_LEVEL`.
        // Passing `None` clears the mock override out of the way so our real production cell gets tested.
        let _guard = MockTerminalGuard::acquire(None);

        set_override(ColorLevel::TrueColor);
        assert_eq!(get_cached_level(), ColorLevel::TrueColor);

        let result = std::panic::catch_unwind(|| {
            set_override(ColorLevel::__Uninitialized);
        });

        assert!(result.is_err());

        // The previous override should still be active.
        assert_eq!(get_cached_level(), ColorLevel::TrueColor);
    }
}
