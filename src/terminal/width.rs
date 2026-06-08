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
pub fn visual_line_width(line: &str) -> usize {
    let mut width = 0;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();

                for ch in chars.by_ref() {
                    if ('@'..='~').contains(&ch) {
                        break;
                    }
                }
            }

            continue;
        }

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

#[cfg(all(test, feature = "unicode-width"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_line_width_plain_ascii() {
        assert_eq!(visual_line_width("Hello"), 5);
    }

    #[test]
    fn test_visual_line_width_empty_string() {
        assert_eq!(visual_line_width(""), 0);
    }

    #[test]
    fn test_visual_line_width_ignores_sgr_color_sequences() {
        assert_eq!(visual_line_width("\x1b[31mRed\x1b[0m"), 3);
    }

    #[test]
    fn test_visual_line_width_ignores_sgr_bold_and_color_sequence() {
        assert_eq!(visual_line_width("\x1b[1;31mWarning\x1b[0m"), 7);
    }

    #[test]
    fn test_visual_line_width_ignores_non_sgr_csi_sequences() {
        assert_eq!(visual_line_width("\x1b[2KHello"), 5);
    }

    #[test]
    fn test_visual_line_width_ignores_cursor_movement_sequences() {
        assert_eq!(visual_line_width("\x1b[HHome"), 4);
    }

    #[test]
    fn test_visual_line_width_counts_emoji_width() {
        assert_eq!(visual_line_width("Hi 🦀"), 5);
    }

    #[test]
    fn test_visual_line_width_counts_cjk_width() {
        assert_eq!(visual_line_width("Hello 华"), 8);
    }

    #[test]
    fn test_visual_line_width_ignores_control_characters() {
        assert_eq!(visual_line_width("A\nB\rC\tD"), 4);
    }

    #[test]
    fn test_visual_line_width_mixed_text_and_escape_sequences() {
        let line = "Status: \x1b[32mOK\x1b[0m \x1b[2Kdone";
        assert_eq!(visual_line_width(line), 15);
    }
}
