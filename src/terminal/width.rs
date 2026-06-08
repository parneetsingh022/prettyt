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

#[cfg(feature = "unicode-width")]
pub(crate) fn truncate_to_visual_width(s: &str, max_width: usize) -> &str {
    let mut width = 0;
    let mut end = 0;

    for (idx, ch) in s.char_indices() {
        let ch_width = visual_line_width(ch.encode_utf8(&mut [0; 4]));

        if width + ch_width > max_width {
            break;
        }

        width += ch_width;
        end = idx + ch.len_utf8();
    }

    &s[..end]
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

    #[test]
    fn test_truncate_ascii_under_limit() {
        assert_eq!(truncate_to_visual_width("Hello", 10), "Hello");
    }

    #[test]
    fn test_truncate_ascii_exact_limit() {
        assert_eq!(truncate_to_visual_width("Hello", 5), "Hello");
    }

    #[test]
    fn test_truncate_ascii_over_limit() {
        assert_eq!(truncate_to_visual_width("Hello", 3), "Hel");
    }

    #[test]
    fn test_truncate_zero_width() {
        assert_eq!(truncate_to_visual_width("Hello", 0), "");
    }

    #[test]
    fn test_truncate_cjk_without_cutting_utf8() {
        assert_eq!(truncate_to_visual_width("Hello 华", 7), "Hello ");
        assert_eq!(truncate_to_visual_width("Hello 华", 8), "Hello 华");
    }

    #[test]
    fn test_truncate_emoji_without_cutting_utf8() {
        assert_eq!(truncate_to_visual_width("Hi 🦀", 3), "Hi ");
        assert_eq!(truncate_to_visual_width("Hi 🦀", 5), "Hi 🦀");
    }

    #[test]
    fn test_truncate_multiple_wide_chars() {
        assert_eq!(truncate_to_visual_width("华🚀A", 2), "华");
        assert_eq!(truncate_to_visual_width("华🚀A", 4), "华🚀");
        assert_eq!(truncate_to_visual_width("华🚀A", 5), "华🚀A");
    }

    #[test]
    fn test_truncate_combining_marks() {
        assert_eq!(truncate_to_visual_width("नमस्ते", 4), "नमस्ते");
        assert_eq!(truncate_to_visual_width("नमस्ते", 3), "नमस्");
    }

    #[test]
    fn test_truncate_empty_string() {
        assert_eq!(truncate_to_visual_width("", 5), "");
    }
}
