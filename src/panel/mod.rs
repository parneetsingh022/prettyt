//! Terminal panel widget for rendering bordered text boxes.
//!
//! This module provides the [`Panel`] type, a lightweight component for
//! displaying text inside a bordered box in the terminal.
//!
//! Panels automatically expand to the current terminal width.
//! If the terminal dimensions cannot be determined,
//! a fallback width of `80` columns is used.
//!
//!
//! # Example
//!
//! ```rust,no_run
//! use prettyt::panel::Panel;
//!
//! let panel = Panel::new("Application started");
//! panel.draw();
//! ```
//!
//! Example output:
//!
//! ```text
//! ╭────────────────────────────╮
//! │Application started         │
//! ╰────────────────────────────╯
//! ```
//!
//! # Notes
//!
//! - Panel width is based on the current terminal width.
//! - Text is rendered on a single line and is not wrapped.
//! - If the text exceeds the available width, it may overflow the panel
//!   boundaries.

use crate::terminal::dimensions::get_terminal_size;

/// A terminal panel containing text to be rendered.
///
/// Create a panel with [`Panel::new`] and display it using [`Panel::draw`].
pub struct Panel<'a> {
    /// The text displayed inside the panel.
    text: &'a str,
}

impl<'a> Panel<'a> {
    /// Creates a new [`Panel`] containing the provided text.
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Draws the panel to standard output.
    /// # Example
    ///
    /// ```rust,no_run
    /// use prettyt::panel::Panel;
    /// let panel = Panel::new("Loading...");
    /// panel.draw();
    /// ```
    pub fn draw(&self) {
        let width = get_terminal_size()
            .map(|dim| dim.width as usize)
            .unwrap_or(80);

        // We need 2 spaces for the vertical borders (│ text │)
        let reserved_space = self.text.len() + 2;

        let text_line_width = width.saturating_sub(reserved_space);

        // Top border with rounded corners: ╭────────────────╮
        println!("╭{:─<width$}╮", "", width = width.saturating_sub(2));

        // Middle text line: │ text         │
        println!("│{}{: <width$}│", self.text, "", width = text_line_width);

        // Bottom border with rounded corners: ╰────────────────╯
        println!("╰{:─<width$}╯", "", width = width.saturating_sub(2));
    }
}
