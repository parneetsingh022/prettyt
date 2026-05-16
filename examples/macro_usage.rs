//! # Macro Usage Example
//!
//! This example showcases how to use the declarative macros provided by `prettyt`
//! for cleaner, inline terminal styling and formatting.

use prettyt::style::Color;
use prettyt::{make_style, println_styled};

fn main() {
    // =========================================================================
    // Building Styles Natively with `make_style!`
    // =========================================================================

    // Combine structural text properties, foreground, and background colors.
    // Notice how trailing commas are seamlessly supported!
    let critical_style = make_style!(fg(Color::RED), bg(Color::BLACK), bold, underline,);

    let subtle_style = make_style!(fg(Color::Ansi256(244)), italic);
    let alert_badge = make_style!(fg(Color::WHITE), bg(Color::RED), invert);

    // Render using manual `.apply(...)` tracking
    println!("{}", critical_style.apply("CRITICAL DATABASE EXCEPTION"));
    println!("{}", subtle_style.apply("Connection pool footprint: 4.2ms"));
    println!("{} File upload pending.", alert_badge.apply(" RETRY "));
    println!();

    // =========================================================================
    // High-Efficiency Inline Printing with `println_styled!`
    // =========================================================================

    let info_style = make_style!(fg(Color::CYAN), bold);
    let success_style = make_style!(fg(Color::GREEN));

    // Seamlessly handles string format interpolation arguments without
    // forcing manual allocation steps beforehand.
    println_styled!(
        info_style,
        "-> Launching cluster workers on local thread context node #{}",
        104
    );

    println_styled!(
        success_style,
        "-> Cluster synchronization status: {} (verified in {}s)",
        "OK",
        0.003
    );
}
