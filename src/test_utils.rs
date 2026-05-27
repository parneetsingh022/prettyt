use crate::terminal::ColorLevel;
use crate::terminal::registry::force_mock_color_level;
use std::sync::{LazyLock, Mutex, MutexGuard};

static TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// RAII guard that serializes test execution crate-wide
/// and guarantees mock state cleanup on drop.
pub struct MockTerminalGuard<'a> {
    _lock: MutexGuard<'a, ()>,
}

impl<'a> MockTerminalGuard<'a> {
    pub fn acquire(level: ColorLevel) -> Self {
        let lock = TEST_MUTEX.lock().unwrap();
        force_mock_color_level(Some(level));

        Self { _lock: lock }
    }
}

impl<'a> Drop for MockTerminalGuard<'a> {
    fn drop(&mut self) {
        force_mock_color_level(None);
    }
}
