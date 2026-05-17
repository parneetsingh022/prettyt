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
