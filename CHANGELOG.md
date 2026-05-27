# Changelog

All notable changes to this project will be documented in this file. This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Removed
* **Global Control Overrides:** Introduced thread-safe `set_override(level: ColorLevel)` and `clear_override()` APIs, allowing developers to programmatically bypass environment cascades and explicitly lock down terminal styling preferences at runtime.
* **Apple Terminal Strikethrough Fallback:** Removed the custom character-stitching fallback loop for macOS `Terminal.app` to resolve layout corruption, string data bloat, and ANSI tracking de-synchronization. The library now universally emits standard SGR 9 escape sequences across all environments.

## [0.2.0] - 2026-05-26

### Added
* **Apple Terminal Strikethrough Fallback:** Implemented an automated fallback engine for macOS Terminal.app that dynamically embeds Unicode combining long stroke characters (`\u{0336}`) through printable text strings to resolve its native lack of `\x1b[9m` ANSI sequence support.

* **Zero-Allocation Lazy Evaluation:** Introduced `StyledRef` to lazily stream ANSI text styling configurations directly to formatting targets via `fmt::Display` implementation, completely eliminating heap allocations on formatting passes.

### Breaking Changes
* **Reference-Bound `Style::apply` API:** The signature of `Style::apply` now accepts arguments by shared reference (`&T`) instead of taking ownership by value. Passing owned primitives or numeric constants manually now explicitly requires a reference borrow prefix (e.g., `style.apply(&34)` instead of `style.apply(34)`). Normal string literals remain unaffected as they naturally satisfy the reference type bound (`&str`).

## [0.1.0] - 2026-05-22

### Added
* **Multi-Tier Colors:** Full support for 16 standard ANSI colors, 256-color palettes, and 24-bit TrueColor RGB.
* **Smart Downsampling:** Automatic fallback math to translate rich RGB colors down to 256 or 16 colors depending on terminal support.
* **Environment Detection:** Strict compliance with `NO_COLOR`, `FORCE_COLOR`, TTY status checking (`stdout.is_terminal()`), `COLORTERM`, and `TERM`.
* **Fluent API:** Builder pattern interface (`Style::new()`) to programmatically stack properties like colors, bold, italic, underline, dim, strikethrough, and inversion.
* **Logging Macros:** `make_style!` for declarative style creation and `sprintln!` for direct formatting output.
* **Performance:** Zero external dependencies and thread-safe environment caching via `OnceLock`.
