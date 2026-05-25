use core::fmt;

use crate::terminal::ColorLevel;
use crate::terminal::registry::get_cached_level;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Foreground,
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    // 16 Standard ANSI colors
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
    // Extended variations
    Rgb(u8, u8, u8),
    Ansi256(u8),
    None,
}

impl Color {
    /// Associated constant array mapping indices 0..=15 directly to Color variants.
    pub(crate) const ANSI16_COLORS: [Color; 16] = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::BrightBlack,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];

    /// Reconstructs a top-level 16-color ANSI variant from a raw u8 index (0..=15).
    ///
    /// # Panics
    /// Panics if the provided index is greater than 15.
    pub(crate) fn ansi16_from_u8(c: u8) -> Color {
        Self::ANSI16_COLORS
            .get(c as usize)
            .copied()
            .unwrap_or_else(|| panic!("ANSI16 color index out of range (expected 0..=15): {c}"))
    }

    pub(crate) fn map_ansi16(&self) -> Option<u8> {
        match self {
            Color::Black => Some(0),
            Color::Red => Some(1),
            Color::Green => Some(2),
            Color::Yellow => Some(3),
            Color::Blue => Some(4),
            Color::Magenta => Some(5),
            Color::Cyan => Some(6),
            Color::White => Some(7),
            Color::BrightBlack => Some(8),
            Color::BrightRed => Some(9),
            Color::BrightGreen => Some(10),
            Color::BrightYellow => Some(11),
            Color::BrightBlue => Some(12),
            Color::BrightMagenta => Some(13),
            Color::BrightCyan => Some(14),
            Color::BrightWhite => Some(15),
            Color::Ansi256(_) | Color::Rgb(_, _, _) | Color::None => None,
        }
    }
}

pub(crate) fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    fn channel(v: u8) -> u8 {
        match v {
            0..=47 => 0,
            48..=114 => 1,
            _ => (v - 35) / 40,
        }
    }

    let r = channel(r);
    let g = channel(g);
    let b = channel(b);

    16 + 36 * r + 6 * g + b
}

pub(crate) fn ansi256_to_ansi16(n: u8) -> Color {
    const ANSI16_RGB: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (205, 49, 49),
        (13, 188, 121),
        (229, 229, 16),
        (36, 114, 200),
        (188, 63, 188),
        (17, 168, 205),
        (229, 229, 229),
        (102, 102, 102),
        (241, 76, 76),
        (35, 209, 139),
        (245, 245, 67),
        (59, 142, 234),
        (214, 112, 214),
        (41, 184, 219),
        (255, 255, 255),
    ];

    let rgb = match n {
        0..=15 => return Color::ansi16_from_u8(n),

        16..=231 => {
            let x = n - 16;
            let r = x / 36;
            let g = (x / 6) % 6;
            let b = x % 6;

            let level = |v: u8| match v {
                0 => 0,
                1 => 95,
                2 => 135,
                3 => 175,
                4 => 215,
                5 => 255,
                _ => unreachable!(),
            };

            (level(r), level(g), level(b))
        }

        232..=255 => {
            let gray = 8 + (n - 232) * 10;

            return match gray {
                0..=54 => Color::Black,
                55..=159 => Color::BrightBlack,
                160..=239 => Color::White,
                _ => Color::BrightWhite,
            };
        }
    };

    let mut best = 0;
    let mut best_dist = u32::MAX;

    for (i, &(cr, cg, cb)) in ANSI16_RGB.iter().enumerate() {
        let dr = rgb.0 - cr as i32;
        let dg = rgb.1 - cg as i32;
        let db = rgb.2 - cb as i32;

        let dist = (dr * dr + dg * dg + db * db) as u32;

        if dist < best_dist {
            best = i as u8;
            best_dist = dist;
        }
    }

    Color::ansi16_from_u8(best)
}

fn get_appropriate_color_for_level(color: Color, level: ColorLevel) -> Color {
    match level {
        ColorLevel::None => Color::None,

        ColorLevel::TrueColor => color,

        ColorLevel::Ansi256 => match color {
            Color::Rgb(r, g, b) => Color::Ansi256(rgb_to_ansi256(r, g, b)),
            _ => color,
        },

        ColorLevel::Basic => match color {
            Color::Rgb(r, g, b) => ansi256_to_ansi16(rgb_to_ansi256(r, g, b)),
            Color::Ansi256(c) => ansi256_to_ansi16(c),
            _ => color,
        },
    }
}

pub(crate) fn get_appropriate_color(color: Color) -> Color {
    get_appropriate_color_for_level(color, get_cached_level())
}

pub(crate) fn to_ansi_string(
    f: &mut fmt::Formatter<'_>,
    color: Color,
    layer: Layer,
) -> fmt::Result {
    let color = get_appropriate_color(color);

    let fg = matches!(layer, Layer::Foreground);

    if let Some(n) = color.map_ansi16() {
        let code = match (fg, n < 8) {
            (true, true) => 30 + n,
            (true, false) => 90 + (n - 8),
            (false, true) => 40 + n,
            (false, false) => 100 + (n - 8),
        };

        return write!(f, "\x1b[{}m", code);
    }

    match color {
        Color::None => Ok(()),

        Color::Rgb(r, g, b) => {
            let code = if fg { 38 } else { 48 };
            write!(f, "\x1b[{};2;{};{};{}m", code, r, g, b)
        }

        Color::Ansi256(v) => {
            let code = if fg { 38 } else { 48 };
            write!(f, "\x1b[{};5;{}m", code, v)
        }

        _ => unreachable!(),
    }
}

#[cfg(test)]
pub(crate) fn to_ansi_string_for_test(color: Color, layer: Layer) -> String {
    struct Wrapper {
        color: Color,
        layer: Layer,
    }
    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            to_ansi_string(f, self.color, self.layer)
        }
    }
    format!("{}", Wrapper { color, layer })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;
    use crate::terminal::{ColorLevel, TerminalApp};
    use crate::test_utils::MockTerminalGuard;
    // !!!! rgb_to_ansi256
    #[test]
    fn rgb_to_ansi256_black() {
        assert_eq!(rgb_to_ansi256(0, 0, 0), 16);
    }

    #[test]
    fn rgb_to_ansi256_white() {
        assert_eq!(rgb_to_ansi256(255, 255, 255), 231);
    }

    #[test]
    fn rgb_to_ansi256_primary_colors() {
        assert_eq!(rgb_to_ansi256(255, 0, 0), 196);
        assert_eq!(rgb_to_ansi256(0, 255, 0), 46);
        assert_eq!(rgb_to_ansi256(0, 0, 255), 21);
    }

    #[test]
    fn rgb_to_ansi256_rounds_to_nearest_channel_level() {
        assert_eq!(rgb_to_ansi256(254, 0, 0), 196);
        assert_eq!(rgb_to_ansi256(204, 0, 0), 160);
        assert_eq!(rgb_to_ansi256(203, 0, 0), 160);
        assert_eq!(rgb_to_ansi256(202, 0, 0), 160);
    }

    #[test]
    fn rgb_to_ansi256_xterm_thresholds() {
        assert_eq!(rgb_to_ansi256(47, 0, 0), 16);
        assert_eq!(rgb_to_ansi256(48, 0, 0), 52);

        assert_eq!(rgb_to_ansi256(114, 0, 0), 52);
        assert_eq!(rgb_to_ansi256(115, 0, 0), 88);

        assert_eq!(rgb_to_ansi256(154, 0, 0), 88);
        assert_eq!(rgb_to_ansi256(155, 0, 0), 124);

        assert_eq!(rgb_to_ansi256(194, 0, 0), 124);
        assert_eq!(rgb_to_ansi256(195, 0, 0), 160);

        assert_eq!(rgb_to_ansi256(234, 0, 0), 160);
        assert_eq!(rgb_to_ansi256(235, 0, 0), 196);
    }

    #[test]
    fn rgb_to_ansi256_xterm_levels() {
        assert_eq!(rgb_to_ansi256(0, 0, 0), 16);
        assert_eq!(rgb_to_ansi256(0, 0, 95), 17);
        assert_eq!(rgb_to_ansi256(0, 0, 135), 18);
        assert_eq!(rgb_to_ansi256(0, 0, 175), 19);
        assert_eq!(rgb_to_ansi256(0, 0, 215), 20);
        assert_eq!(rgb_to_ansi256(0, 0, 255), 21);
    }

    #[test]
    fn rgb_to_ansi256_mixed_color() {
        assert_eq!(rgb_to_ansi256(128, 64, 255), 99);
    }

    // !!!! ansi256_to_ansi16
    #[test]
    fn ansi256_to_ansi16_keeps_ansi16_values() {
        let expected = [
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::BrightBlack,
            Color::BrightRed,
            Color::BrightGreen,
            Color::BrightYellow,
            Color::BrightBlue,
            Color::BrightMagenta,
            Color::BrightCyan,
            Color::BrightWhite,
        ];

        for (n, color) in expected.into_iter().enumerate() {
            assert_eq!(ansi256_to_ansi16(n as u8), color);
        }
    }

    #[test]
    fn ansi256_to_ansi16_maps_obvious_colors() {
        assert_eq!(ansi256_to_ansi16(16), Color::Black);
        assert!(matches!(
            ansi256_to_ansi16(196),
            Color::Red | Color::BrightRed
        ));
        assert!(matches!(
            ansi256_to_ansi16(46),
            Color::Green | Color::BrightGreen
        ));
        assert!(matches!(
            ansi256_to_ansi16(21),
            Color::Blue | Color::BrightBlue
        ));
        assert!(matches!(
            ansi256_to_ansi16(226),
            Color::Yellow | Color::BrightYellow
        ));
        assert!(matches!(
            ansi256_to_ansi16(201),
            Color::Magenta | Color::BrightMagenta
        ));
        assert!(matches!(
            ansi256_to_ansi16(51),
            Color::Cyan | Color::BrightCyan
        ));
    }

    fn is_gray(c: Color) -> bool {
        matches!(
            c,
            Color::Black | Color::BrightBlack | Color::White | Color::BrightWhite
        )
    }

    #[test]
    fn grayscale_ramp_maps_to_grayscale_colors() {
        for n in 232..=255 {
            assert!(
                is_gray(ansi256_to_ansi16(n)),
                "ansi256 {n} should map to grayscale, got {:?}",
                ansi256_to_ansi16(n)
            );
        }
    }

    #[test]
    fn ansi256_to_ansi16_never_panics_for_all_values() {
        for n in 0..=255 {
            let _ = ansi256_to_ansi16(n);
        }
    }

    #[test]
    fn warm_rgb_does_not_collapse_to_white() {
        let color = ansi256_to_ansi16(rgb_to_ansi256(214, 108, 92));

        assert!(
            !matches!(color, Color::White | Color::BrightWhite),
            "warm reddish color should not map to white, got {:?}",
            color
        );
    }
    // !!!! to_ansi_string

    #[test]
    fn rgb_foreground() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::Rgb(255, 128, 0), Layer::Foreground),
            "\x1b[38;2;255;128;0m"
        );
    }

    #[test]
    fn rgb_background() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::Rgb(255, 128, 0), Layer::Background),
            "\x1b[48;2;255;128;0m"
        );
    }

    #[test]
    fn ansi256_foreground() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::Ansi256(196), Layer::Foreground),
            "\x1b[38;5;196m"
        );
    }
    #[test]
    fn ansi256_background() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::Ansi256(196), Layer::Background),
            "\x1b[48;5;196m"
        );
    }

    #[test]
    fn ansi16_normal_foreground() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::Red, Layer::Foreground),
            "\x1b[31m"
        );
    }

    #[test]
    fn ansi16_normal_background() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::Red, Layer::Background),
            "\x1b[41m"
        );
    }

    #[test]
    fn ansi16_bright_foreground() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::BrightRed, Layer::Foreground),
            "\x1b[91m"
        );
    }

    #[test]
    fn ansi16_bright_background() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        assert_eq!(
            to_ansi_string_for_test(Color::BrightRed, Layer::Background),
            "\x1b[101m"
        );
    }

    #[test]
    fn ansi16_boundaries() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);

        assert_eq!(
            to_ansi_string_for_test(Color::Black, Layer::Foreground),
            "\x1b[30m"
        );
        assert_eq!(
            to_ansi_string_for_test(Color::White, Layer::Foreground),
            "\x1b[37m"
        );
        assert_eq!(
            to_ansi_string_for_test(Color::BrightBlack, Layer::Foreground),
            "\x1b[90m"
        );
        assert_eq!(
            to_ansi_string_for_test(Color::BrightWhite, Layer::Foreground),
            "\x1b[97m"
        );

        assert_eq!(
            to_ansi_string_for_test(Color::Black, Layer::Background),
            "\x1b[40m"
        );
        assert_eq!(
            to_ansi_string_for_test(Color::White, Layer::Background),
            "\x1b[47m"
        );
        assert_eq!(
            to_ansi_string_for_test(Color::BrightBlack, Layer::Background),
            "\x1b[100m"
        );
        assert_eq!(
            to_ansi_string_for_test(Color::BrightWhite, Layer::Background),
            "\x1b[107m"
        );
    }

    // !!! get_appropriate_color_for_level

    #[test]
    fn returns_none_when_color_level_is_none() {
        assert_eq!(
            get_appropriate_color_for_level(Color::Rgb(255, 0, 0), ColorLevel::None),
            Color::None
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::Ansi256(196), ColorLevel::None),
            Color::None
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::Red, ColorLevel::None),
            Color::None
        );
    }

    #[test]
    fn truecolor_keeps_original_color() {
        assert_eq!(
            get_appropriate_color_for_level(Color::Rgb(128, 64, 255), ColorLevel::TrueColor),
            Color::Rgb(128, 64, 255)
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::Ansi256(196), ColorLevel::TrueColor),
            Color::Ansi256(196)
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::Red, ColorLevel::TrueColor),
            Color::Red
        );
    }

    #[test]
    fn ansi256_converts_rgb_to_ansi256() {
        assert_eq!(
            get_appropriate_color_for_level(Color::Rgb(255, 0, 0), ColorLevel::Ansi256),
            Color::Ansi256(196)
        );
    }

    #[test]
    fn ansi256_keeps_ansi256_and_ansi16() {
        assert_eq!(
            get_appropriate_color_for_level(Color::Ansi256(196), ColorLevel::Ansi256),
            Color::Ansi256(196)
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::Red, ColorLevel::Ansi256),
            Color::Red
        );
    }

    #[test]
    fn basic_converts_rgb_and_ansi256_to_ansi16() {
        assert!(matches!(
            get_appropriate_color_for_level(Color::Rgb(255, 0, 0), ColorLevel::Basic),
            Color::Red | Color::BrightRed
        ));

        assert!(matches!(
            get_appropriate_color_for_level(Color::Ansi256(196), ColorLevel::Basic),
            Color::Red | Color::BrightRed
        ));
    }

    #[test]
    fn basic_keeps_ansi16() {
        assert_eq!(
            get_appropriate_color_for_level(Color::Red, ColorLevel::Basic),
            Color::Red
        );
    }

    #[test]
    fn color_none_stays_none_for_color_capable_levels() {
        assert_eq!(
            get_appropriate_color_for_level(Color::None, ColorLevel::TrueColor),
            Color::None
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::None, ColorLevel::Ansi256),
            Color::None
        );

        assert_eq!(
            get_appropriate_color_for_level(Color::None, ColorLevel::Basic),
            Color::None
        );
    }
}
