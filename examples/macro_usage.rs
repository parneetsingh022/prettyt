//! # Macro Usage Example
//!
//! This example showcases how to use the declarative macros provided by `prettyt`
//! for cleaner, inline terminal styling and formatting.

use prettyt::{Color, make_style, sprintln};

fn main() {
    // =========================================================================
    // Building Styles Natively with `make_style!`
    // =========================================================================

    // Combine structural text properties, foreground, and background colors.
    // Notice how trailing commas are seamlessly supported!
    let critical_style = make_style!(fg(Color::Red), bg(Color::Black), bold, underline,);

    let subtle_style = make_style!(fg(Color::Ansi256(244)), italic);
    let alert_badge = make_style!(fg(Color::White), bg(Color::Red), invert);

    // Render using manual `.apply(...)` tracking
    println!("{}", critical_style.apply("CRITICAL DATABASE EXCEPTION"));
    println!("{}", subtle_style.apply("Connection pool footprint: 4.2ms"));
    println!("{} File upload pending.", alert_badge.apply(" RETRY "));
    println!();

    // =========================================================================
    // High-Efficiency Inline Printing with `sprintln!`
    // =========================================================================

    let info_style = make_style!(fg(Color::Cyan), bold);
    let success_style = make_style!(fg(Color::Green));

    // Seamlessly handles string format interpolation arguments without
    // forcing manual allocation steps beforehand.
    sprintln!(
        info_style,
        "-> Launching cluster workers on local thread context node #{}",
        104
    );

    sprintln!(
        success_style,
        "-> Cluster synchronization status: {} (verified in {}s)",
        "OK",
        0.003
    );
}
