mod terminal;

use crate::terminal::{ColorLevel, detect_color_level};
fn main() {
    let level = detect_color_level();

    match level {
        ColorLevel::None => println!("No color"),
        ColorLevel::Basic => println!("Basic color"),
        ColorLevel::Ansi256 => println!("256 color"),
        ColorLevel::TrueColor => println!("True color"),
    }
}