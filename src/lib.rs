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
//! ### Using Macros
//! ```rust
//! use prettyt::{make_style, println_styled};
//! use prettyt::style::Color;
//!
//! # fn main() {
//! let success_banner = make_style!(fg(Color::BRIGHT_GREEN), bold);
//! println!("{}", success_banner.apply("BUILD SUCCESSFUL"));
//! println_styled!(success_banner, "Success: {}", 101);
//! # }
//! ```
//!
//! ### Using the Builder API
//! ```rust
//! use prettyt::style::{Style, Color};
//!
//! # fn main() {
//! let error_style = Style::new().fg(Color::Rgb(255, 50, 50)).bold().underline();
//! println!("{}", error_style.apply("CRITICAL FAILURE"));
//! # }
//! ```

pub mod style;
pub mod terminal;
