#[cfg(feature = "unicode-width")]
use unicode_width::UnicodeWidthChar;

#[cfg(feature = "terminal_size")]
use terminal_size::{Width, terminal_size};

// =========================================================================
// Zero-Allocation Visual Width Helper
// =========================================================================
/// Counts the visual cell width of a line, completely ignoring ANSI styling
/// sequences while respecting multi-byte or wide unicode characters (emojis/CJK).
#[cfg(feature = "unicode-width")]
pub(crate) fn visual_line_width(line: &str) -> usize {
    let mut width = 0;
    let mut in_ansi = false;

    for c in line.chars() {
        if c == '\x1b' {
            in_ansi = true;
            continue;
        }
        if in_ansi {
            if c == 'm' {
                in_ansi = false;
            }
            continue;
        }

        // unicode-width returns None for non-printable control characters (\n, \r, etc.)
        width += UnicodeWidthChar::width(c).unwrap_or(0);
    }

    width
}

#[cfg(feature = "terminal_size")]
pub(crate) fn terminal_width() -> usize {
    match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80, // fallback
    }
}
