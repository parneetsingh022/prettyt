use unicode_width::UnicodeWidthChar;

// =========================================================================
// Zero-Allocation Visual Width Helper
// =========================================================================
/// Counts the visual cell width of a line, completely ignoring ANSI styling
/// sequences while respecting multi-byte or wide unicode characters (emojis/CJK).
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
