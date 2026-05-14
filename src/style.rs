use std::sync::OnceLock;

use crate::terminal::{ColorLevel, detect_color_level};

static COLOR_SUPPORT: OnceLock<ColorLevel> = OnceLock::new();

/// Returns the detected terminal color support level, caching it after the first call.
///
/// Uses a thread-safe, lazy initialization to check environment variables and TTY status
/// once per program execution.
pub fn get_cached_level() -> ColorLevel {
    *COLOR_SUPPORT.get_or_init(detect_color_level)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Ansi(u8),
    Basic(BasicColor),
}

pub fn ansi16_to_basic(n: u8) -> BasicColor {
    match n {
        0 => BasicColor::Black,
        1 => BasicColor::Red,
        2 => BasicColor::Green,
        3 => BasicColor::Yellow,
        4 => BasicColor::Blue,
        5 => BasicColor::Magenta,
        6 => BasicColor::Cyan,
        7 => BasicColor::White,
        8 => BasicColor::BrightBlack,
        9 => BasicColor::BrightRed,
        10 => BasicColor::BrightGreen,
        11 => BasicColor::BrightYellow,
        12 => BasicColor::BrightBlue,
        13 => BasicColor::BrightMagenta,
        14 => BasicColor::BrightCyan,
        15 => BasicColor::BrightWhite,
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let r: u8 = r / 51;
    let g = g / 51;
    let b = b / 51;

    16 + 36 * r + 6 * g + b
}

#[allow(dead_code)]
fn ansi256_to_ansi16(n: u8) -> BasicColor {
    let idx: u8 = match n {
        0..=15 => n,

        // grayscale ramp
        232..=255 => {
            let gray = 8 + (n - 232) * 10;
            if gray < 128 { 0 } else { 15 }
        }

        // 6x6x6 color cube
        16..=231 => {
            let x = n - 16;
            let r = x / 36;
            let g = (x / 6) % 6;
            let b = x % 6;

            let bright = r >= 3 || g >= 3 || b >= 3;

            let base = if r >= g && r >= b {
                // yellow
                if g >= 3 && b < 3 {
                    3
                }
                // magenta
                else if b >= 3 && g < 3 {
                    5
                }
                // red
                else {
                    1
                }
            } else if g >= r && g >= b {
                // yellow
                if r >= 3 && b < 3 {
                    3
                }
                // cyan
                else if b >= 3 && r < 3 {
                    6
                }
                // green
                else {
                    2
                }
            } else {
                // magenta
                if r >= 3 && g < 3 {
                    5
                }
                // cyan
                else if g >= 3 && r < 3 {
                    6
                }
                // blue
                else {
                    4
                }
            };

            if bright { base + 8 } else { base }
        }
    };

    ansi16_to_basic(idx)
}

fn to_ansi_string(color: Color, background: bool) -> String {
    match color {
        Color::Rgb(r, g, b) => {
            let code = if background { 48 } else { 38 };
            format!("\x1b[{};2;{};{};{}m", code, r, g, b)
        }
        _ => unimplemented!(),
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);

        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);

        self
    }

    pub fn apply(&self, value: impl std::fmt::Display) -> String {
        let mut prefix = String::new();

        if let Some(color) = self.fg {
            prefix.push_str(&to_ansi_string(color, false));
        }

        if let Some(color) = self.bg {
            prefix.push_str(&to_ansi_string(color, true));
        }

        if prefix.is_empty() {
            value.to_string()
        } else {
            format!("{}{}\x1b[0m", prefix, value)
        }
    }
}
