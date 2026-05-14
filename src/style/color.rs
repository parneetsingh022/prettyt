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

#[allow(dead_code)]
pub(crate) fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Rounding to the nearest increment of 51
    let r = ((r as u16 * 5 + 127) / 255) as u8;
    let g = ((g as u16 * 5 + 127) / 255) as u8;
    let b = ((b as u16 * 5 + 127) / 255) as u8;

    16 + 36 * r + 6 * g + b
}

#[allow(dead_code)]
pub(crate) fn ansi256_to_ansi16(n: u8) -> BasicColor {
    const ANSI16_RGB: [(u8, u8, u8); 16] = [
        (0, 0, 0),       // black
        (128, 0, 0),     // red
        (0, 128, 0),     // green
        (128, 128, 0),   // yellow
        (0, 0, 128),     // blue
        (128, 0, 128),   // magenta
        (0, 128, 128),   // cyan
        (192, 192, 192), // white / light gray
        (128, 128, 128), // bright black / dark gray
        (255, 0, 0),     // bright red
        (0, 255, 0),     // bright green
        (255, 255, 0),   // bright yellow
        (0, 0, 255),     // bright blue
        (255, 0, 255),   // bright magenta
        (0, 255, 255),   // bright cyan
        (255, 255, 255), // bright white
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
            (gray, gray, gray)
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

pub(crate) fn to_ansi_string(color: Color, layer: Layer) -> String {
    let background = layer == Layer::Background;
    let code = if background { 48 } else { 38 };

    match color {
        Color::Rgb(r, g, b) => {
            format!("\x1b[{};2;{};{};{}m", code, r, g, b)
        }
        Color::Ansi256(v) => {
            format!("\x1b[{};5;{}m", code, v)
        }
        Color::Ansi16(c) => {
            let n = c as u8;

            let code = match (background, n >= 8) {
                (false, false) => 30 + n,
                (false, true) => 90 + (n - 8),
                (true, false) => 40 + n,
                (true, true) => 100 + (n - 8),
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
    fn rgb_to_ansi256_rounding_thresholds() {
        assert_eq!(rgb_to_ansi256(25, 0, 0), 16);
        assert_eq!(rgb_to_ansi256(26, 0, 0), 52);

        assert_eq!(rgb_to_ansi256(76, 0, 0), 52);
        assert_eq!(rgb_to_ansi256(77, 0, 0), 88);

        assert_eq!(rgb_to_ansi256(127, 0, 0), 88);
        assert_eq!(rgb_to_ansi256(128, 0, 0), 124);

        assert_eq!(rgb_to_ansi256(178, 0, 0), 124);
        assert_eq!(rgb_to_ansi256(179, 0, 0), 160);

        assert_eq!(rgb_to_ansi256(229, 0, 0), 160);
        assert_eq!(rgb_to_ansi256(230, 0, 0), 196);
    }

    #[test]
    fn rgb_to_ansi256_mixed_color() {
        assert_eq!(rgb_to_ansi256(128, 64, 255), 135);
    }

    // !!!! ansi256_to_ansi16

    #[test]
    fn ansi256_to_ansi16_keeps_basic_ansi_colors() {
        assert_eq!(ansi256_to_ansi16(0), BasicColor::Black);
        assert_eq!(ansi256_to_ansi16(1), BasicColor::Red);
        assert_eq!(ansi256_to_ansi16(2), BasicColor::Green);
        assert_eq!(ansi256_to_ansi16(3), BasicColor::Yellow);
        assert_eq!(ansi256_to_ansi16(4), BasicColor::Blue);
        assert_eq!(ansi256_to_ansi16(5), BasicColor::Magenta);
        assert_eq!(ansi256_to_ansi16(6), BasicColor::Cyan);
        assert_eq!(ansi256_to_ansi16(7), BasicColor::White);
    }

    #[test]
    fn ansi256_to_ansi16_keeps_bright_ansi_colors() {
        assert_eq!(ansi256_to_ansi16(8), BasicColor::BrightBlack);
        assert_eq!(ansi256_to_ansi16(9), BasicColor::BrightRed);
        assert_eq!(ansi256_to_ansi16(10), BasicColor::BrightGreen);
        assert_eq!(ansi256_to_ansi16(11), BasicColor::BrightYellow);
        assert_eq!(ansi256_to_ansi16(12), BasicColor::BrightBlue);
        assert_eq!(ansi256_to_ansi16(13), BasicColor::BrightMagenta);
        assert_eq!(ansi256_to_ansi16(14), BasicColor::BrightCyan);
        assert_eq!(ansi256_to_ansi16(15), BasicColor::BrightWhite);
    }

    #[test]
    fn ansi256_to_ansi16_maps_color_cube_primaries() {
        assert_eq!(ansi256_to_ansi16(16), BasicColor::Black);
        assert_eq!(ansi256_to_ansi16(196), BasicColor::BrightRed);
        assert_eq!(ansi256_to_ansi16(46), BasicColor::BrightGreen);
        assert_eq!(ansi256_to_ansi16(21), BasicColor::BrightBlue);
    }

    #[test]
    fn ansi256_to_ansi16_maps_color_cube_secondary_colors() {
        assert_eq!(ansi256_to_ansi16(226), BasicColor::BrightYellow);
        assert_eq!(ansi256_to_ansi16(201), BasicColor::BrightMagenta);
        assert_eq!(ansi256_to_ansi16(51), BasicColor::BrightCyan);
    }

    #[test]
    fn ansi256_to_ansi16_maps_dark_color_cube_values() {
        assert_eq!(ansi256_to_ansi16(52), BasicColor::Red);
        assert_eq!(ansi256_to_ansi16(22), BasicColor::Green);
        assert_eq!(ansi256_to_ansi16(17), BasicColor::Blue);
    }

    #[test]
    fn ansi256_to_ansi16_maps_grayscale_ramp() {
        assert_eq!(ansi256_to_ansi16(232), BasicColor::Black);
        assert_eq!(ansi256_to_ansi16(244), BasicColor::BrightBlack);
        assert_eq!(ansi256_to_ansi16(250), BasicColor::White);
        assert_eq!(ansi256_to_ansi16(255), BasicColor::BrightWhite);
    }

    #[test]
    fn ansi256_to_ansi16_maps_neutral_cube_values_to_grays() {
        assert_eq!(ansi256_to_ansi16(59), BasicColor::BrightBlack);
        assert_eq!(ansi256_to_ansi16(102), BasicColor::BrightBlack);
        assert_eq!(ansi256_to_ansi16(145), BasicColor::White);
        assert_eq!(ansi256_to_ansi16(188), BasicColor::White);
        assert_eq!(ansi256_to_ansi16(231), BasicColor::BrightWhite);
    }

    // !!!! to_ansi_string

    #[test]
    fn rgb_foreground() {
        assert_eq!(
            to_ansi_string(Color::Rgb(255, 128, 0), Layer::Foreground),
            "\x1b[38;2;255;128;0m"
        );
    }

    #[test]
    fn rgb_background() {
        assert_eq!(
            to_ansi_string(Color::Rgb(255, 128, 0), Layer::Background),
            "\x1b[48;2;255;128;0m"
        );
    }

    #[test]
    fn ansi256_foreground() {
        assert_eq!(
            to_ansi_string(Color::Ansi256(196), Layer::Foreground),
            "\x1b[38;5;196m"
        );
    }

    #[test]
    fn ansi256_background() {
        assert_eq!(
            to_ansi_string(Color::Ansi256(196), Layer::Background),
            "\x1b[48;5;196m"
        );
    }

    #[test]
    fn ansi16_normal_foreground() {
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::Red), Layer::Foreground),
            "\x1b[31m"
        );
    }

    #[test]
    fn ansi16_normal_background() {
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::Red), Layer::Background),
            "\x1b[41m"
        );
    }

    #[test]
    fn ansi16_bright_foreground() {
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::BrightRed), Layer::Foreground),
            "\x1b[91m"
        );
    }

    #[test]
    fn ansi16_bright_background() {
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::BrightRed), Layer::Background),
            "\x1b[101m"
        );
    }

    #[test]
    fn ansi16_boundaries() {
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::Black), Layer::Foreground),
            "\x1b[30m"
        );
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::White), Layer::Foreground),
            "\x1b[37m"
        );
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::BrightBlack), Layer::Foreground),
            "\x1b[90m"
        );
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::BrightWhite), Layer::Foreground),
            "\x1b[97m"
        );

        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::Black), Layer::Background),
            "\x1b[40m"
        );
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::White), Layer::Background),
            "\x1b[47m"
        );
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::BrightBlack), Layer::Background),
            "\x1b[100m"
        );
        assert_eq!(
            to_ansi_string(Color::Ansi16(BasicColor::BrightWhite), Layer::Background),
            "\x1b[107m"
        );
    }
}
