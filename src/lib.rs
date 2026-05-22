//! # `prettyt`
//!
//! A lightweight, zero-dependency, environment-aware ANSI terminal text styling library
//! featuring automatic color capability downsampling.
//!
//! ## Core Features
//! * **Automatic Detection**: Dynamically inspects the terminal environment (see the [`terminal`] submodule) to safeguard layouts.
//! * **Smart Downsampling**: Gracefully downsamples `TrueColor` (RGB) colors to 256-color palettes or basic 16-color ANSI buckets depending on what the host terminal supports.
//! * **Zero Dependencies**: Keeps your dependency tree completely flat and compiles exceptionally fast.
//! * **Fluent & Declarative APIs**: Choose between a builder-pattern `Style` struct or clean, compile-checked macros.
//!
//! ## Quick Start
//!
//! ### Inline Printing (Using Macros)
//! Use the `println_styled!` macro to cleanly format and print styled text. It seamlessly handles native `format!` interpolation arguments directly, eliminating the need to manually build separate formatted strings beforehand:
//! ```rust
//!
//! use prettyt::{Color, make_style, println_styled};
//!
//! # fn main() {
//! let info = make_style!(fg(Color::BrightCyan), bold);
//! let success = make_style!(fg(Color::BrightGreen));
//!
//! // Pass format arguments smoothly into the macro
//! println_styled!(info, "-> Launching cluster workers on node #{}", 104);
//!
//! println_styled!(
//!     success,
//!     "-> Status: {} (verified in {}s)",
//!     "OK",
//!     0.003
//! );
//! # }
//! ```
//!
//! ### Builder Style Formatting
//! For finer control, you can build styles dynamically using the fluent builder API. The `.apply()` method accepts any type implementing `std::fmt::Display` (such as strings, integers, or floats) and returns an environment-aware styled string:
//!
//! ```rust
//! use prettyt::{Style, Color};
//!
//! # fn main() {
//! let error_badge = Style::new().fg(Color::White).bg(Color::Red).bold();
//! let highlight = Style::new().fg(Color::Cyan).bold();
//!
//! // Pass strings or numeric values seamlessly to .apply()
//! println!("{} Database panic!", error_badge.apply(" PANIC "));
//! println!("Returned error code: {}", highlight.apply(500));
//! # }
//! ```

pub mod style;
pub mod terminal;

pub use style::{Color, Style};
