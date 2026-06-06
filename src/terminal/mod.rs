//! Terminal color capability detection and environment handling.
//!
//! This module determines the terminal's supported color level by inspecting
//! environment variables and whether stdout is attached to an interactive TTY.
//! The detected [`ColorLevel`] is used throughout `prettyt` to ensure styled
//! output behaves correctly across terminals, pipes, log files, and CI systems.
//!
//! # Detection Order
//!
//! Detection uses a strict precedence-based cascade. The first matching rule
//! determines the final [`ColorLevel`].
//!
//! | Priority | Source | Behavior | Result |
//! | :---: | :--- | :--- | :--- |
//! | **1** | `NO_COLOR` | Variable is present (any value) | [`ColorLevel::None`] |
//! | **2** | `FORCE_COLOR` | `"0"` | [`ColorLevel::None`] |
//! | | | `"1"` | [`ColorLevel::Basic`] |
//! | | | `"2"` | [`ColorLevel::Ansi256`] |
//! | | | Any other value (`"3"`, `"true"`, `"yes"`, etc.) | [`ColorLevel::TrueColor`] |
//! | **3** | TTY Check (`stdout`) | Output is not a terminal | [`ColorLevel::None`] |
//! | **4** | `COLORTERM` | Contains `"truecolor"` or `"24bit"` | [`ColorLevel::TrueColor`] |
//! | **5** | `TERM` | Equals `"dumb"` | [`ColorLevel::None`] |
//! | | | Contains `"256color"` | [`ColorLevel::Ansi256`] |
//! | **6** | Fallback | Standard interactive terminal | [`ColorLevel::Basic`] |
//!
//! ---
//!
//! # Behavior Notes
//!
//! ## `NO_COLOR`
//!
//! `NO_COLOR` has the highest precedence and always disables styling,
//! regardless of TTY state or forced color settings. This follows the
//! community convention defined at [no-color.org](https://no-color.org).
//!
//! ## `FORCE_COLOR`
//!
//! `FORCE_COLOR` explicitly enables color output, even when stdout is being
//! piped or redirected. This is commonly used in CI pipelines, snapshot tests,
//! and logging environments where ANSI output should be preserved.
//!
//! ## TTY Detection
//!
//! If stdout is not connected to an interactive terminal and no explicit
//! override is active, styling is disabled automatically to avoid leaking raw
//! ANSI escape sequences into logs or downstream commands.
//!
//! ## `COLORTERM` and `TERM`
//!
//! Modern terminals often expose `COLORTERM=truecolor` or `24bit` to advertise
//! full 24-bit RGB support. Legacy terminals commonly expose `256color` through
//! the `TERM` variable to indicate extended ANSI palette support.
pub mod detect;
pub mod registry;

pub(crate) mod width;

#[cfg(feature = "unicode-width")]
pub(crate) use width::visual_line_width;

#[cfg(feature = "terminal_size")]
pub(crate) use width::terminal_width;

pub use detect::{ColorLevel, detect_color_level};
pub(crate) use registry::get_cached_level;
