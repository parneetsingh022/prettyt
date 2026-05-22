<div align="center">

# prettyt

[![Rust Tests](https://github.com/parneetsingh022/prettyt/actions/workflows/test.yml/badge.svg)](https://github.com/parneetsingh022/prettyt/actions/workflows/test.yml)
[![Codecov](https://codecov.io/gh/parneetsingh022/prettyt/graph/badge.svg)](https://codecov.io/gh/parneetsingh022/prettyt) 
[![Crates.io](https://img.shields.io/crates/v/prettyt.svg)](https://crates.io/crates/prettyt) 
[![Docs.rs](https://docs.rs/prettyt/badge.svg)](https://docs.rs/prettyt) 
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

A lightweight, environment-aware terminal text styling library with automatic color downsampling.

<img width="671" height="277" alt="image" src="https://github.com/user-attachments/assets/f31cd8d5-b2ed-44b7-bbd2-e2034bd498e6" />
</div>


## Features
* ANSI16, ANSI256, and TrueColor support
* Automatic terminal color capability detection
* NO_COLOR support
* Fluent builder-style API
* Declarative styling macros
* Zero dependencies
* Lightweight and fast

## Quick Start

### Inline Printing (Using Macros)
Use the `sprintln!` macro to cleanly format and print styled text. It seamlessly handles native `format!` interpolation arguments directly, eliminating the need to manually build separate formatted strings beforehand:
```rust
use prettyt::{Color, make_style, sprintln};

fn main() {
    let info = make_style!(fg(Color::BrightCyan), bold);
    let success = make_style!(fg(Color::BrightGreen));

    // Pass format arguments smoothly into the macro
    sprintln!(info, "-> Launching cluster workers on node #{}", 104);
    
    sprintln!(
        success, 
        "-> Status: {} (verified in {}s)", 
        "OK", 
        0.003
    );
}
```
### Builder Style Formatting
For finer control, you can build styles dynamically using the fluent builder API. The `.apply()` method accepts any type implementing `std::fmt::Display` (such as strings, integers, or floats) and returns an environment-aware styled string:

```rust

use prettyt::{Style, Color};

fn main() {
    let error_badge = Style::new().fg(Color::White).bg(Color::Red).bold();
    let highlight = Style::new().fg(Color::Cyan).bold();

    // Pass strings or numeric values seamlessly to .apply()
    println!("{} Database panic!", error_badge.apply(" PANIC "));
    println!("Returned error code: {}", highlight.apply(500));
}

```

## Available Styles


| Style | Macro Syntax | ANSI Code |
|:---:|:---:|:---:|
| Bold | `bold` | `\x1b[1m` |
| Dim | `dim` | `\x1b[2m` |
| Italic | `italic` | `\x1b[3m` |
| Underline | `underline` | `\x1b[4m` |
| Invert | `invert` | `\x1b[7m` |
| Strikethrough | `strikethrough` | `\x1b[9m` |
| Foreground Color | `fg(Color::RED)` | `\x1b[3Xm` / extended |
| Background Color | `bg(Color::BLUE)` | `\x1b[4Xm` / extended |

---

## Colors
### ANSI16
```rust
Color::RED
Color::GREEN
Color::BLUE
Color::BRIGHT_CYAN
```

### ANSI256
```rust
Color::Ansi256(196)
```

### TrueColor RGB
```rust
Color::Rgb(255, 120, 0)
```


## Combining Styles
```rust
use prettyt::{Color, make_style};

fn main() {
    let style = make_style!(
        fg(Color::BRIGHT_MAGENTA),
        bg(Color::BLACK),
        bold,
        underline
    );

    println!("{}", style.apply("PrettyT"));
}
```

## Environment Configuration & Overrides

`prettyt` automatically detects terminal color support based on the current
environment and output stream. This behavior can be customized using standard
environment variables:

| Environment Variable | Allowed Values | Description |
|---|---|---|
| **`NO_COLOR`** | Any value | Disables all styling and color output. Presence alone is enough to disable colors, following the [no-color.org](https://no-color.org) convention. |
| **`FORCE_COLOR`** | `0`, `1`, `2`, `3`, `true`, `yes` | Explicitly forces a color level, even when output is redirected or piped. `0` disables color, `1` enables basic ANSI colors, `2` enables 256-color support, and any other value enables TrueColor. |
| **`COLORTERM`** | `truecolor`, `24bit` | Indicates native 24-bit RGB color support in the active terminal emulator. |
| **`TERM`** | `dumb`, `*256color*` | Fallback terminal capability detection. `dumb` disables styling, while values containing `256color` enable 256-color support. |

### Detection Precedence

Environment detection follows this priority order:

1. `NO_COLOR`
2. `FORCE_COLOR`
3. TTY detection (`stdout.is_terminal()`)
4. `COLORTERM`
5. `TERM`
6. Fallback to basic ANSI colors

### Piping & Redirection

If stdout is redirected or piped to another process, `prettyt` automatically
disables styling to avoid leaking ANSI escape sequences into logs or plain-text
outputs. This behavior can still be overridden with `FORCE_COLOR`.
