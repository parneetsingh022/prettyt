//! # Global Override Example
//!
//! This example demonstrates how to programmatically override the automatic
//! terminal capability detection at runtime using `set_override` and `clear_override`.

use prettyt::terminal::ColorLevel;
use prettyt::{Color, Style, clear_override, set_override};

fn main() {
    // Create a striking style for critical system logs
    let critical_alert = Style::new().fg(Color::BrightRed).bg(Color::Black).bold();
    let success_badge = Style::new().fg(Color::BrightGreen).bold();

    // =========================================================================
    // Default Behavior (Environment Cascade)
    // =========================================================================
    println!("--- Default Environment Mode ---");
    println!(
        "{} System initialization sequence started.",
        success_badge.apply("[ OK ]")
    );
    println!();

    // =========================================================================
    // Force Plain-Text Mode (e.g., Exporting Clean Logs to a File)
    // =========================================================================
    println!("--- Enforcing Plain-Text Mode (ColorLevel::None) ---");

    // Globally strip all ANSI styling sequences from this point forward
    set_override(ColorLevel::None);

    println!(
        "{} Core dumped or piped downstream without ANSI code leaks.",
        critical_alert.apply("[ CRITICAL ]")
    );
    println!(
        "{} This badge is also safely unstyled.",
        success_badge.apply("[ OK ]")
    );
    println!();

    // =========================================================================
    // Force Extended Color Spectrum (e.g., Strict CI/CD Snapshot Tests)
    // =========================================================================
    println!("--- Enforcing Extended Colors (ColorLevel::Ansi256) ---");

    // Manually clamp the environment to a specific color palette depth
    set_override(ColorLevel::Ansi256);

    println!(
        "{} Falling back cleanly to fixed 256-color palette constraints.",
        critical_alert.apply("[ CRITICAL ]")
    );
    println!();

    // =========================================================================
    // Restore Control Back to the Environment
    // =========================================================================
    println!("--- Restoring Automatic Terminal Detection ---");

    // Clear the override and fall back to automatic (cached) terminal detection
    clear_override();

    println!(
        "{} Standard environment capability detection is back online.",
        success_badge.apply("[ OK ]")
    );
}
