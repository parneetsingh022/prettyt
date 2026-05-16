use crate::terminal::ColorLevel;
use crate::terminal::registry::get_cached_level;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    BrightBlack = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Foreground,
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Ansi256(u8),
    Ansi16(BasicColor),
    None,
}

impl Color {
    pub const BLACK: Color = Color::Ansi16(BasicColor::Black);
    pub const RED: Color = Color::Ansi16(BasicColor::Red);
    pub const GREEN: Color = Color::Ansi16(BasicColor::Green);
    pub const YELLOW: Color = Color::Ansi16(BasicColor::Yellow);
    pub const BLUE: Color = Color::Ansi16(BasicColor::Blue);
    pub const MAGENTA: Color = Color::Ansi16(BasicColor::Magenta);
    pub const CYAN: Color = Color::Ansi16(BasicColor::Cyan);
    pub const WHITE: Color = Color::Ansi16(BasicColor::White);

    pub const BRIGHT_BLACK: Color = Color::Ansi16(BasicColor::BrightBlack);
    pub const BRIGHT_RED: Color = Color::Ansi16(BasicColor::BrightRed);
    pub const BRIGHT_GREEN: Color = Color::Ansi16(BasicColor::BrightGreen);
    pub const BRIGHT_YELLOW: Color = Color::Ansi16(BasicColor::BrightYellow);
    pub const BRIGHT_BLUE: Color = Color::Ansi16(BasicColor::BrightBlue);
    pub const BRIGHT_MAGENTA: Color = Color::Ansi16(BasicColor::BrightMagenta);
    pub const BRIGHT_CYAN: Color = Color::Ansi16(BasicColor::BrightCyan);
    pub const BRIGHT_WHITE: Color = Color::Ansi16(BasicColor::BrightWhite);
}

pub(crate) fn ansi16_to_basic(n: u8) -> BasicColor {
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

pub(crate) fn ansi256_to_ansi16(n: u8) -> BasicColor {
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
        0..=15 => return ansi16_to_basic(n),

        16..=231 => {
            let x = n - 16;
            let r = x / 36;
            let g = (x / 6) % 6;
            let b = x % 6;

            let level = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };

            (level(r), level(g), level(b))
        }

        232..=255 => {
            let gray = 8 + (n - 232) * 10;

            return match gray {
                0..=64 => BasicColor::Black,
                65..=159 => BasicColor::BrightBlack,
                160..=239 => BasicColor::White,
                _ => BasicColor::BrightWhite,
            };
        }
    };

    let mut best = 0;
    let mut best_dist = u32::MAX;

    for (i, &(cr, cg, cb)) in ANSI16_RGB.iter().enumerate() {
        let dr = rgb.0 as i32 - cr as i32;
        let dg = rgb.1 as i32 - cg as i32;
        let db = rgb.2 as i32 - cb as i32;

        let dist = (dr * dr + dg * dg + db * db) as u32;

        if dist < best_dist {
            best = i as u8;
            best_dist = dist;
        }
    }

    ansi16_to_basic(best)
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
            Color::Rgb(r, g, b) => Color::Ansi16(ansi256_to_ansi16(rgb_to_ansi256(r, g, b))),
            Color::Ansi256(c) => Color::Ansi16(ansi256_to_ansi16(c)),
            _ => color,
        },
    }
}

pub(crate) fn get_appropriate_color(color: Color) -> Color {
    get_appropriate_color_for_level(color, get_cached_level())
}

pub(crate) fn to_ansi_string(color: Color, layer: Layer) -> String {
    let color = get_appropriate_color(color);

    to_ansi_string_inner(color, layer)
}

pub(crate) fn to_ansi_string_inner(color: Color, layer: Layer) -> String {
    let fg = matches!(layer, Layer::Foreground);

    match color {
        Color::None => String::new(),

        Color::Rgb(r, g, b) => {
            let code = if fg { 38 } else { 48 };
            format!("\x1b[{};2;{};{};{}m", code, r, g, b)
        }

        Color::Ansi256(v) => {
            let code = if fg { 38 } else { 48 };
            format!("\x1b[{};5;{}m", code, v)
        }

        Color::Ansi16(c) => {
            let n = c as u8;

            let code = match (fg, n < 8) {
                (true, true) => 30 + n,
                (true, false) => 90 + (n - 8),
                (false, true) => 40 + n,
                (false, false) => 100 + (n - 8),
            };

            format!("\x1b[{}m", code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            BasicColor::Black,
            BasicColor::Red,
            BasicColor::Green,
            BasicColor::Yellow,
            BasicColor::Blue,
            BasicColor::Magenta,
            BasicColor::Cyan,
            BasicColor::White,
            BasicColor::BrightBlack,
            BasicColor::BrightRed,
            BasicColor::BrightGreen,
            BasicColor::BrightYellow,
            BasicColor::BrightBlue,
            BasicColor::BrightMagenta,
            BasicColor::BrightCyan,
            BasicColor::BrightWhite,
        ];

        for (n, color) in expected.into_iter().enumerate() {
            assert_eq!(ansi256_to_ansi16(n as u8), color);
        }
    }

    #[test]
    fn ansi256_to_ansi16_maps_obvious_colors() {
        assert_eq!(ansi256_to_ansi16(16), BasicColor::Black);
        assert!(matches!(
            ansi256_to_ansi16(196),
            BasicColor::Red | BasicColor::BrightRed
        ));
        assert!(matches!(
            ansi256_to_ansi16(46),
            BasicColor::Green | BasicColor::BrightGreen
        ));
        assert!(matches!(
            ansi256_to_ansi16(21),
            BasicColor::Blue | BasicColor::BrightBlue
        ));
        assert!(matches!(
            ansi256_to_ansi16(226),
            BasicColor::Yellow | BasicColor::BrightYellow
        ));
        assert!(matches!(
            ansi256_to_ansi16(201),
            BasicColor::Magenta | BasicColor::BrightMagenta
        ));
        assert!(matches!(
            ansi256_to_ansi16(51),
            BasicColor::Cyan | BasicColor::BrightCyan
        ));
    }

    fn is_gray(c: BasicColor) -> bool {
        matches!(
            c,
            BasicColor::Black
                | BasicColor::BrightBlack
                | BasicColor::White
                | BasicColor::BrightWhite
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
            !matches!(color, BasicColor::White | BasicColor::BrightWhite),
            "warm reddish color should not map to white, got {:?}",
            color
        );
    }
    // !!!! to_ansi_string

    #[test]
    fn rgb_foreground() {
        assert_eq!(
            to_ansi_string_inner(Color::Rgb(255, 128, 0), Layer::Foreground),
            "\x1b[38;2;255;128;0m"
        );
    }

    #[test]
    fn rgb_background() {
        assert_eq!(
            to_ansi_string_inner(Color::Rgb(255, 128, 0), Layer::Background),
            "\x1b[48;2;255;128;0m"
        );
    }

    #[test]
    fn ansi256_foreground() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi256(196), Layer::Foreground),
            "\x1b[38;5;196m"
        );
    }

    #[test]
    fn ansi256_background() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi256(196), Layer::Background),
            "\x1b[48;5;196m"
        );
    }

    #[test]
    fn ansi16_normal_foreground() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::Red), Layer::Foreground),
            "\x1b[31m"
        );
    }

    #[test]
    fn ansi16_normal_background() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::Red), Layer::Background),
            "\x1b[41m"
        );
    }

    #[test]
    fn ansi16_bright_foreground() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::BrightRed), Layer::Foreground),
            "\x1b[91m"
        );
    }

    #[test]
    fn ansi16_bright_background() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::BrightRed), Layer::Background),
            "\x1b[101m"
        );
    }

    #[test]
    fn ansi16_boundaries() {
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::Black), Layer::Foreground),
            "\x1b[30m"
        );
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::White), Layer::Foreground),
            "\x1b[37m"
        );
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::BrightBlack), Layer::Foreground),
            "\x1b[90m"
        );
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::BrightWhite), Layer::Foreground),
            "\x1b[97m"
        );

        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::Black), Layer::Background),
            "\x1b[40m"
        );
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::White), Layer::Background),
            "\x1b[47m"
        );
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::BrightBlack), Layer::Background),
            "\x1b[100m"
        );
        assert_eq!(
            to_ansi_string_inner(Color::Ansi16(BasicColor::BrightWhite), Layer::Background),
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
            get_appropriate_color_for_level(Color::Ansi16(BasicColor::Red), ColorLevel::None),
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
            get_appropriate_color_for_level(Color::Ansi16(BasicColor::Red), ColorLevel::TrueColor),
            Color::Ansi16(BasicColor::Red)
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
            get_appropriate_color_for_level(Color::Ansi16(BasicColor::Red), ColorLevel::Ansi256),
            Color::Ansi16(BasicColor::Red)
        );
    }

    #[test]
    fn basic_converts_rgb_and_ansi256_to_ansi16() {
        assert!(matches!(
            get_appropriate_color_for_level(Color::Rgb(255, 0, 0), ColorLevel::Basic),
            Color::Ansi16(BasicColor::Red | BasicColor::BrightRed)
        ));

        assert!(matches!(
            get_appropriate_color_for_level(Color::Ansi256(196), ColorLevel::Basic),
            Color::Ansi16(BasicColor::Red | BasicColor::BrightRed)
        ));
    }

    #[test]
    fn basic_keeps_ansi16() {
        assert_eq!(
            get_appropriate_color_for_level(Color::Ansi16(BasicColor::Red), ColorLevel::Basic),
            Color::Ansi16(BasicColor::Red)
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
