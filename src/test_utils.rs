//! Test utilities for isolating terminal color state.
//!
//! The terminal color registry uses process-wide global state for cached color
//! detection and runtime overrides. Because Rust tests run in parallel by
//! default, tests that read or mutate that state can interfere with each other
//! unless they are serialized.
//!
//! This module provides [`MockTerminalGuard`], an RAII guard that:
//!
//! - serializes terminal-color-related tests with a crate-wide mutex,
//! - resets cached terminal color state before each guarded test,
//! - optionally installs a temporary [`ColorLevel`] override,
//! - clears/reset cached state again when the guard is dropped.
//!
//! Passing `Some(level)` to [`MockTerminalGuard::acquire`] forces that terminal
//! color level for the duration of the test. Passing `None` clears the override
//! and allows tests to exercise the real detection/cache path.
//!
//! The global mutex is poison-tolerant because it only protects test execution
//! order, not application data.

use crate::clear_override;
use crate::terminal::ColorLevel;
use crate::terminal::registry::{reset_cached_level, set_override};
use std::sync::{LazyLock, Mutex, MutexGuard};

static TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// RAII guard that serializes test execution crate-wide
/// and guarantees mock state cleanup on drop.
pub(crate) struct MockTerminalGuard<'a> {
    _lock: MutexGuard<'a, ()>,
}

impl<'a> MockTerminalGuard<'a> {
    pub(crate) fn acquire(level: impl Into<Option<ColorLevel>>) -> Self {
        let lock = TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        reset_cached_level();
        let level = level.into();

        match level {
            Some(level) => set_override(level),
            None => clear_override(),
        }

        Self { _lock: lock }
    }
}

impl<'a> Drop for MockTerminalGuard<'a> {
    fn drop(&mut self) {
        reset_cached_level();
    }
}
