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
```rust
use prettyt::style::Color;
use prettyt::make_style;

fn main() {
    let success = make_style!(fg(Color::BRIGHT_GREEN), bold);
    let error = make_style!(fg(Color::BRIGHT_RED), bold);

    println!("{}", success.apply("SUCCESS"));
    println!("{}", error.apply("ERROR"));
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
use prettyt::style::Color;
use prettyt::make_style;

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

`prettyt` automatically determines the appropriate color capability depth depending on your user's environment. You can explicitly customize or override this runtime behavior using standard environment variables:

| Environment Variable | Allowed Values | Description |
|---|---|---|
| **`FORCE_COLOR`** | `0`, `1`, `2`, `3` or `yes` | Explicitly overrides all other checks (including piped output and `NO_COLOR`). `0` turns color off, `1` targets basic 16 colors, `2` targets 256 colors, and `3`/`yes` forces TrueColor. |
| **`NO_COLOR`** | Any value | Disables styling outright (per the [no-color.org](https://no-color.org) standard) unless overridden by `FORCE_COLOR`. |
| **`COLORTERM`** | `truecolor`, `24bit` | Signals to `prettyt` that the terminal emulator natively handles raw 24-bit RGB values. |
| **`TERM`** | `dumb`, `*256color*` | Base fallback identification. `dumb` suppresses style codes, while strings containing `256color` safely enable the 256-index lookup downsampler. |

### Piping & Redirection Support
By default, if `stdout` is redirected or piped to another process or file (meaning it is not an interactive TTY), `prettyt` automatically disables styling to keep raw output text clean and pristine, unless manually forced via `FORCE_COLOR`.