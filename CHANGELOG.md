# Changelog

All notable changes to this project will be documented in this file. This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-22

### Added
* **Multi-Tier Colors:** Full support for 16 standard ANSI colors, 256-color palettes, and 24-bit TrueColor RGB.
* **Smart Downsampling:** Automatic fallback math to translate rich RGB colors down to 256 or 16 colors depending on terminal support.
* **Environment Detection:** Strict compliance with `NO_COLOR`, `FORCE_COLOR`, TTY status checking (`stdout.is_terminal()`), `COLORTERM`, and `TERM`.
* **Fluent API:** Builder pattern interface (`Style::new()`) to programmatically stack properties like colors, bold, italic, underline, dim, strikethrough, and inversion.
* **Logging Macros:** `make_style!` for declarative style creation and `sprintln!` for direct formatting output.
* **Performance:** Zero external dependencies and thread-safe environment caching via `OnceLock`.
