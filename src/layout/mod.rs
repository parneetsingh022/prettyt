//! Layout primitives for composing structured terminal output.
//!
//! This module provides high-level building blocks for arranging and formatting
//! terminal content. These components abstract away raw styling APIs, offering
//! reusable structures like panels and text blocks to simplify UI construction.
//!
//! > To use layout primitives like `Panel` or `Text`, you must explicitly
//! > enable the `layout` feature in your `Cargo.toml`.
//!
//! # Components
//!
//! When the `layout` feature is enabled, the following components become available:
//!
//! | Component | Description                                                   |
//! | :---      | :---                                                          |
//! | [`panel`] | Draws text framed inside a customizable, bordered box.        |
//! | [`text`]  | A text widget that can be displayed within the layout system. |
//!
//! # Examples
//!
//! Creating and displaying a simple bordered panel:
//!
//! ```rust,no_run
//! use prettyt::layout::{Panel, Text};
//!
//! let text = Text::new("Hello from prettyt");
//! let panel = Panel::new(&text);
//!
//! // Prints a framed message to the terminal
//! println!("{}", panel);
//! ```
use core::fmt;

pub mod panel;
pub mod text;

pub use panel::Panel;
pub use text::Text;

pub struct SizeHint {
    pub min: usize,
    pub max: Option<usize>,
}

pub trait Renderable {
    fn measure(&self, max_width: usize) -> SizeHint;
    fn total_rows(&self, width: usize) -> usize;

    /// Returns the visible character width of a specific row when wrapped to `width`.
    fn row_width(&self, row_idx: usize, width: usize) -> usize;

    /// Streams a row directly to the concrete Formatter slot.
    fn render_row(&self, row_idx: usize, width: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// A zero-allocation proxy container that binds a [`Renderable`] element to an explicit
/// rendering width, enabling lazy evaluations during streaming output passes.
pub struct LayoutDisplay<'a, T: Renderable> {
    pub layout: &'a T,
    pub width: usize,
}

impl<'a, T: Renderable> fmt::Display for LayoutDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = self.layout.total_rows(self.width);
        for i in 0..total {
            self.layout.render_row(i, self.width, f)?;
            if i < total - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}
