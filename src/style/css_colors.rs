use crate::Color;

/// A collection of standard CSS named colors.
///
/// This type provides constants for all CSS Color Module Level 4 named colors,
/// allowing colors to be referenced without manually specifying hexadecimal
/// values.
///
/// # Example
///
/// ```rust
/// use prettyt::CSSColor;
///
/// let background = CSSColor::ALICE_BLUE;
/// let accent = CSSColor::ROYAL_BLUE;
/// ```
///
/// All color values match the official CSS specification and are stored as
/// [`Color`] instances.
///
/// # Naming Convention
///
/// CSS color names are exposed as uppercase snake case constants:
///
/// | CSS Name | Rust Constant |
/// |-----------|---------------|
/// | `aliceblue` | `ALICE_BLUE` |
/// | `cornflowerblue` | `CORNFLOWER_BLUE` |
/// | `rebeccapurple` | `REBECCA_PURPLE` |
///
/// Some CSS colors have synonymous names and therefore share the same value:
///
/// - `AQUA` = `CYAN`
/// - `FUCHSIA` = `MAGENTA`
/// - `GRAY` = `GREY`
/// - `DARK_GRAY` = `DARK_GREY`
/// - `LIGHT_GRAY` = `LIGHT_GREY`
/// - `SLATE_GRAY` = `SLATE_GREY`
/// - `DIM_GRAY` = `DIM_GREY`
///
/// # Specification
///
/// Color values are based on the CSS Color Module Level 4 named color
/// definitions:
/// <https://www.w3.org/TR/css-color-4/#named-colors>
pub struct CSSColor;

impl CSSColor {
    pub const ALICE_BLUE: Color = Color::from_hex("#F0F8FF");
    pub const ANTIQUE_WHITE: Color = Color::from_hex("#FAEBD7");
    pub const AQUA: Color = Color::from_hex("#00FFFF");
    pub const AQUAMARINE: Color = Color::from_hex("#7FFFD4");
    pub const AZURE: Color = Color::from_hex("#F0FFFF");
    pub const BEIGE: Color = Color::from_hex("#F5F5DC");
    pub const BISQUE: Color = Color::from_hex("#FFE4C4");
    pub const BLACK: Color = Color::from_hex("#000000");
    pub const BLANCHED_ALMOND: Color = Color::from_hex("#FFEBCD");
    pub const BLUE: Color = Color::from_hex("#0000FF");
    pub const BLUE_VIOLET: Color = Color::from_hex("#8A2BE2");
    pub const BROWN: Color = Color::from_hex("#A52A2A");
    pub const BURLY_WOOD: Color = Color::from_hex("#DEB887");
    pub const CADET_BLUE: Color = Color::from_hex("#5F9EA0");
    pub const CHARTREUSE: Color = Color::from_hex("#7FFF00");
    pub const CHOCOLATE: Color = Color::from_hex("#D2691E");
    pub const CORAL: Color = Color::from_hex("#FF7F50");
    pub const CORNFLOWER_BLUE: Color = Color::from_hex("#6495ED");
    pub const CORNSILK: Color = Color::from_hex("#FFF8DC");
    pub const CRIMSON: Color = Color::from_hex("#DC143C");
    pub const CYAN: Color = Color::from_hex("#00FFFF");
    pub const DARK_BLUE: Color = Color::from_hex("#00008B");
    pub const DARK_CYAN: Color = Color::from_hex("#008B8B");
    pub const DARK_GOLDENROD: Color = Color::from_hex("#B8860B");
    pub const DARK_GRAY: Color = Color::from_hex("#A9A9A9");
    pub const DARK_GREY: Color = Color::from_hex("#A9A9A9");
    pub const DARK_GREEN: Color = Color::from_hex("#006400");
    pub const DARK_KHAKI: Color = Color::from_hex("#BDB76B");
    pub const DARK_MAGENTA: Color = Color::from_hex("#8B008B");
    pub const DARK_OLIVE_GREEN: Color = Color::from_hex("#556B2F");
    pub const DARK_ORANGE: Color = Color::from_hex("#FF8C00");
    pub const DARK_ORCHID: Color = Color::from_hex("#9932CC");
    pub const DARK_RED: Color = Color::from_hex("#8B0000");
    pub const DARK_SALMON: Color = Color::from_hex("#E9967A");
    pub const DARK_SEA_GREEN: Color = Color::from_hex("#8FBC8F");
    pub const DARK_SLATE_BLUE: Color = Color::from_hex("#483D8B");
    pub const DARK_SLATE_GRAY: Color = Color::from_hex("#2F4F4F");
    pub const DARK_SLATE_GREY: Color = Color::from_hex("#2F4F4F");
    pub const DARK_TURQUOISE: Color = Color::from_hex("#00CED1");
    pub const DARK_VIOLET: Color = Color::from_hex("#9400D3");
    pub const DEEP_PINK: Color = Color::from_hex("#FF1493");
    pub const DEEP_SKY_BLUE: Color = Color::from_hex("#00BFFF");
    pub const DIM_GRAY: Color = Color::from_hex("#696969");
    pub const DIM_GREY: Color = Color::from_hex("#696969");
    pub const DODGER_BLUE: Color = Color::from_hex("#1E90FF");
    pub const FIREBRICK: Color = Color::from_hex("#B22222");
    pub const FLORAL_WHITE: Color = Color::from_hex("#FFFAF0");
    pub const FOREST_GREEN: Color = Color::from_hex("#228B22");
    pub const FUCHSIA: Color = Color::from_hex("#FF00FF");
    pub const GAINSBORO: Color = Color::from_hex("#DCDCDC");
    pub const GHOST_WHITE: Color = Color::from_hex("#F8F8FF");
    pub const GOLD: Color = Color::from_hex("#FFD700");
    pub const GOLDENROD: Color = Color::from_hex("#DAA520");
    pub const GRAY: Color = Color::from_hex("#808080");
    pub const GREY: Color = Color::from_hex("#808080");
    pub const GREEN: Color = Color::from_hex("#008000");
    pub const GREEN_YELLOW: Color = Color::from_hex("#ADFF2F");
    pub const HONEYDEW: Color = Color::from_hex("#F0FFF0");
    pub const HOT_PINK: Color = Color::from_hex("#FF69B4");
    pub const INDIAN_RED: Color = Color::from_hex("#CD5C5C");
    pub const INDIGO: Color = Color::from_hex("#4B0082");
    pub const IVORY: Color = Color::from_hex("#FFFFF0");
    pub const KHAKI: Color = Color::from_hex("#F0E68C");
    pub const LAVENDER: Color = Color::from_hex("#E6E6FA");
    pub const LAVENDER_BLUSH: Color = Color::from_hex("#FFF0F5");
    pub const LAWN_GREEN: Color = Color::from_hex("#7CFC00");
    pub const LEMON_CHIFFON: Color = Color::from_hex("#FFFACD");
    pub const LIGHT_BLUE: Color = Color::from_hex("#ADD8E6");
    pub const LIGHT_CORAL: Color = Color::from_hex("#F08080");
    pub const LIGHT_CYAN: Color = Color::from_hex("#E0FFFF");
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color::from_hex("#FAFAD2");
    pub const LIGHT_GRAY: Color = Color::from_hex("#D3D3D3");
    pub const LIGHT_GREY: Color = Color::from_hex("#D3D3D3");
    pub const LIGHT_GREEN: Color = Color::from_hex("#90EE90");
    pub const LIGHT_PINK: Color = Color::from_hex("#FFB6C1");
    pub const LIGHT_SALMON: Color = Color::from_hex("#FFA07A");
    pub const LIGHT_SEA_GREEN: Color = Color::from_hex("#20B2AA");
    pub const LIGHT_SKY_BLUE: Color = Color::from_hex("#87CEFA");
    pub const LIGHT_SLATE_GRAY: Color = Color::from_hex("#778899");
    pub const LIGHT_SLATE_GREY: Color = Color::from_hex("#778899");
    pub const LIGHT_STEEL_BLUE: Color = Color::from_hex("#B0C4DE");
    pub const LIGHT_YELLOW: Color = Color::from_hex("#FFFFE0");
    pub const LIME: Color = Color::from_hex("#00FF00");
    pub const LIME_GREEN: Color = Color::from_hex("#32CD32");
    pub const LINEN: Color = Color::from_hex("#FAF0E6");
    pub const MAGENTA: Color = Color::from_hex("#FF00FF");
    pub const MAROON: Color = Color::from_hex("#800000");
    pub const MEDIUM_AQUAMARINE: Color = Color::from_hex("#66CDAA");
    pub const MEDIUM_BLUE: Color = Color::from_hex("#0000CD");
    pub const MEDIUM_ORCHID: Color = Color::from_hex("#BA55D3");
    pub const MEDIUM_PURPLE: Color = Color::from_hex("#9370DB");
    pub const MEDIUM_SEA_GREEN: Color = Color::from_hex("#3CB371");
    pub const MEDIUM_SLATE_BLUE: Color = Color::from_hex("#7B68EE");
    pub const MEDIUM_SPRING_GREEN: Color = Color::from_hex("#00FA9A");
    pub const MEDIUM_TURQUOISE: Color = Color::from_hex("#48D1CC");
    pub const MEDIUM_VIOLET_RED: Color = Color::from_hex("#C71585");
    pub const MIDNIGHT_BLUE: Color = Color::from_hex("#191970");
    pub const MINT_CREAM: Color = Color::from_hex("#F5FFFA");
    pub const MISTY_ROSE: Color = Color::from_hex("#FFE4E1");
    pub const MOCCASIN: Color = Color::from_hex("#FFE4B5");
    pub const NAVAJO_WHITE: Color = Color::from_hex("#FFDEAD");
    pub const NAVY: Color = Color::from_hex("#000080");
    pub const OLD_LACE: Color = Color::from_hex("#FDF5E6");
    pub const OLIVE: Color = Color::from_hex("#808000");
    pub const OLIVE_DRAB: Color = Color::from_hex("#6B8E23");
    pub const ORANGE: Color = Color::from_hex("#FFA500");
    pub const ORANGE_RED: Color = Color::from_hex("#FF4500");
    pub const ORCHID: Color = Color::from_hex("#DA70D6");
    pub const PALE_GOLDENROD: Color = Color::from_hex("#EEE8AA");
    pub const PALE_GREEN: Color = Color::from_hex("#98FB98");
    pub const PALE_TURQUOISE: Color = Color::from_hex("#AFEEEE");
    pub const PALE_VIOLET_RED: Color = Color::from_hex("#DB7093");
    pub const PAPAYA_WHIP: Color = Color::from_hex("#FFEFD5");
    pub const PEACH_PUFF: Color = Color::from_hex("#FFDAB9");
    pub const PERU: Color = Color::from_hex("#CD853F");
    pub const PINK: Color = Color::from_hex("#FFC0CB");
    pub const PLUM: Color = Color::from_hex("#DDA0DD");
    pub const POWDER_BLUE: Color = Color::from_hex("#B0E0E6");
    pub const PURPLE: Color = Color::from_hex("#800080");
    pub const REBECCA_PURPLE: Color = Color::from_hex("#663399");
    pub const RED: Color = Color::from_hex("#FF0000");
    pub const ROSY_BROWN: Color = Color::from_hex("#BC8F8F");
    pub const ROYAL_BLUE: Color = Color::from_hex("#4169E1");
    pub const SADDLE_BROWN: Color = Color::from_hex("#8B4513");
    pub const SALMON: Color = Color::from_hex("#FA8072");
    pub const SANDY_BROWN: Color = Color::from_hex("#F4A460");
    pub const SEA_GREEN: Color = Color::from_hex("#2E8B57");
    pub const SEASHELL: Color = Color::from_hex("#FFF5EE");
    pub const SIENNA: Color = Color::from_hex("#A0522D");
    pub const SILVER: Color = Color::from_hex("#C0C0C0");
    pub const SKY_BLUE: Color = Color::from_hex("#87CEEB");
    pub const SLATE_BLUE: Color = Color::from_hex("#6A5ACD");
    pub const SLATE_GRAY: Color = Color::from_hex("#708090");
    pub const SLATE_GREY: Color = Color::from_hex("#708090");
    pub const SNOW: Color = Color::from_hex("#FFFAFA");
    pub const SPRING_GREEN: Color = Color::from_hex("#00FF7F");
    pub const STEEL_BLUE: Color = Color::from_hex("#4682B4");
    pub const TAN: Color = Color::from_hex("#D2B48C");
    pub const TEAL: Color = Color::from_hex("#008080");
    pub const THISTLE: Color = Color::from_hex("#D8BFD8");
    pub const TOMATO: Color = Color::from_hex("#FF6347");
    pub const TURQUOISE: Color = Color::from_hex("#40E0D0");
    pub const VIOLET: Color = Color::from_hex("#EE82EE");
    pub const WHEAT: Color = Color::from_hex("#F5DEB3");
    pub const WHITE: Color = Color::from_hex("#FFFFFF");
    pub const WHITE_SMOKE: Color = Color::from_hex("#F5F5F5");
    pub const YELLOW: Color = Color::from_hex("#FFFF00");
    pub const YELLOW_GREEN: Color = Color::from_hex("#9ACD32");
}
