//! Terminal color capability detection and environment routing.
//!
//! This module handles the core environmental discovery cascade for `prettyt`.
//! It dynamically inspects active environment variables alongside the host's standard
//! output stream configuration to negotiate an accurate [`ColorLevel`]. This safeguards downstream formatting and layout engines from polluting logs or breaking basic displays.
//!
//! # The Detection Cascade
//!
//! Resolution operates sequentially using a strict short-circuiting cascade. The first rule to match completely dictates the resulting terminal capability level:
//!
//! | Priority | Mechanism / Variable | Matching Rule & Behavior | Target Level |
//! | :---: | :--- | :--- | :--- |
//! | **1** | `FORCE_COLOR` | Matches `"0"` | [`ColorLevel::None`] |
//! | | | Matches `"1"` | [`ColorLevel::Basic`] |
//! | | | Matches `"2"` | [`ColorLevel::Ansi256`] |
//! | | | Matches any other value (e.g., `"3"`, `"yes"`) | [`ColorLevel::TrueColor`] |
//! | **2** | TTY Check (`stdout`) | Stream is piped or redirected (`!is_terminal()`) | [`ColorLevel::None`] |
//! | **3** | `NO_COLOR` | Variable is present in environment (any value) | [`ColorLevel::None`] |
//! | **4** | `COLORTERM` | Contains substring `"truecolor"` or `"24bit"` | [`ColorLevel::TrueColor`] |
//! | **5** | `TERM` | Value equals `"dumb"` | [`ColorLevel::None`] |
//! | | | Contains substring `"256color"` | [`ColorLevel::Ansi256`] |
//! | **6** | *Fallback* | Default catch-all rule for standard TTY contexts | [`ColorLevel::Basic`] |
//!
//! ---
//!
//! # Behavioral Rules & Context
//!
//! ### 1. Color Explicit Overrides (`FORCE_COLOR`)
//! The `FORCE_COLOR` environment variable acts as a master override lever. When configured, **all interactive TTY evaluations and standard `NO_COLOR` provisions are ignored entirely**. This is uniquely beneficial for continuous integration (CI) platforms, build runners, or test pipelines where styled streams need to be preserved inside automated logs.
//!
//! ### 2. Pipeline Safeguarding (TTY Detection)
//! When standard output (`stdout`) is actively piped to a subsequent utility (such as `grep` or `cat`) or dumped directly into a log file, the terminal framework flags the channel as a non-interactive text stream. `prettyt` dynamically drops styling markers out of the layout under these circumstances to ensure raw outputs remain clean and text logs are not corrupted with raw escape sequence characters.
//!
//! ### 3. Accessibility Compliance (`NO_COLOR`)
//! Consistently enforces community standard profiles defined by [no-color.org](https://no-color.org). If a user exports `NO_COLOR` inside their operating shell config, `prettyt` strips layout rendering back to raw values to respect local visibility, contrast, and screen-reader software demands.
//!
//! ### 4. Modern TrueColor Layouts (`COLORTERM` & `TERM`)
//! Advanced display managers (e.g., Alacritty, iTerm2, VS Code integrated consoles) populate `COLORTERM` to indicate support for comprehensive 24-bit RGB processing channels. When discovered, `prettyt` handles colors with pinpoint fidelity. For standard legacy devices containing `256color` in their environment strings, high-performance math vectors automatically downsample TrueColor values down into the closest indexing slot within the terminal's 256-color cube.

pub mod detect;
pub mod registry;

pub use detect::{ColorLevel, detect_color_level};
pub(crate) use registry::get_cached_level;
