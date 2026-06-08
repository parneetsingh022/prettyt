//! Draws text framed inside a customizable, bordered box.
//!
//! `Panel` wraps another layout component in a box border and can display an
//! optional title in the top border.
//!
//! Panels can also be nested to create more complex layouts.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust
//! use prettyt::layout::{Panel, Text};
//!
//! let text = Text::new("Hello");
//! let panel = Panel::new(&text).title("Greeting");
//!
//! println!("{}", panel);
//! ```
//!
//! Nested panels:
//!
//! ```rust
//! use prettyt::layout::{Panel, Text};
//!
//! let text = Text::new("Hello");
//! let inner = Panel::new(&text).title("Inner");
//! let outer = Panel::new(&inner).title("Outer");
//!
//! println!("{}", outer);
//! ```

use crate::layout::{LayoutDisplay, Renderable, SizeHint};
use crate::terminal::{terminal_width, visual_line_width};
use core::fmt;

/// A bordered container around renderable content.
pub struct Panel<'a, T: Renderable> {
    /// The content displayed inside the panel.
    pub content: &'a T,

    /// The title shown in the top border (Optional).
    pub title: Option<&'a str>,
}

impl<'a, T: Renderable> Panel<'a, T> {
    /// Creates a new panel around the given content.
    pub fn new(content: &'a T) -> Self {
        Self {
            content,
            title: None,
        }
    }

    /// Appends an optional descriptive top border header title.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

impl<'a, T: Renderable> fmt::Display for Panel<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // temporary display configuration proxy
        let display = LayoutDisplay {
            layout: self,
            width: terminal_width(),
        };

        fmt::Display::fmt(&display, f)
    }
}

impl<'a, T: Renderable> Renderable for Panel<'a, T> {
    fn measure(&self, max_width: usize) -> SizeHint {
        let inner_hint = self.content.measure(max_width.saturating_sub(2));
        SizeHint {
            min: inner_hint.min + 2,
            max: inner_hint.max.map(|m| m + 2),
        }
    }

    fn total_rows(&self, width: usize) -> usize {
        self.content.total_rows(width.saturating_sub(2)) + 2
    }

    fn row_width(&self, _row_idx: usize, width: usize) -> usize {
        // A panel always spans its full allocated layout width target

        width
    }

    fn render_row(&self, row_idx: usize, width: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_width = width.saturating_sub(2);
        let total_lines = self.total_rows(width);

        if row_idx == 0 {
            // Top Border
            if let Some(title) = self.title {
                let max_title_len = content_width.saturating_sub(2);
                let truncated_title = if visual_line_width(title) > max_title_len {
                    &title[..max_title_len]
                } else {
                    title
                };

                f.write_str("┌─")?;
                f.write_str(truncated_title)?;
                f.write_str("─")?;

                let remaining = content_width.saturating_sub(visual_line_width(title) + 2);
                for _ in 0..remaining {
                    f.write_str("─")?;
                }
                f.write_str("┐")?;
            } else {
                f.write_str("┌")?;
                for _ in 0..content_width {
                    f.write_str("─")?;
                }
                f.write_str("┐")?;
            }
        } else if row_idx == total_lines - 1 {
            // Bottom Border
            f.write_str("└")?;
            for _ in 0..content_width {
                f.write_str("─")?;
            }
            f.write_str("┘")?;
        } else {
            // Middle Content Rows
            f.write_str("│")?;

            // Render the inner content row directly into standard formatter
            let inner_row_idx = row_idx - 1;
            self.content.render_row(inner_row_idx, content_width, f)?;

            // Query the child's explicit layout width to calculate padding spaces
            let child_row_width = self.content.row_width(inner_row_idx, content_width);
            let padding = content_width.saturating_sub(child_row_width);

            for _ in 0..padding {
                f.write_str(" ")?;
            }

            f.write_str("│")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Text;

    // Helper wrapper to render a specific row index to a String buffer
    struct RowRenderer<'a, T: Renderable>(&'a T, usize, usize);
    impl<'a, T: Renderable> fmt::Display for RowRenderer<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.render_row(self.1, self.2, f)
        }
    }

    // =========================================================================
    // Panel Unit Tests
    // =========================================================================

    #[test]
    fn test_panel_new_defaults() {
        let content = Text::new("hello");
        let panel = Panel::new(&content);

        assert!(panel.title.is_none());
    }

    #[test]
    fn test_panel_fluent_title() {
        let content = Text::new("hello");
        let panel = Panel::new(&content).title("My Title");

        assert_eq!(panel.title, Some("My Title"));
    }

    #[test]
    fn test_panel_geometry_measure() {
        // Content needs 10 slots maximum
        let content = Text::new("1234567890");

        let panel = Panel::new(&content);

        // Panel must expand the inner hint by 2 horizontally for borders
        let hint = panel.measure(40);
        assert_eq!(hint.min, 12);
        assert_eq!(hint.max, Some(12));

        // Ensure clamping works if max_width constraints force it
        let clamped_hint = panel.measure(8);
        assert_eq!(clamped_hint.min, 8);
        assert_eq!(clamped_hint.max, Some(8));
    }

    #[test]
    fn test_panel_total_rows() {
        let content = Text::new("Line 1\nLine 2\nLine 3");

        let panel = Panel::new(&content);

        // Panel expands vertical height by 2 (1 top border + 1 bottom border)
        assert_eq!(panel.total_rows(20), 5);
    }

    #[test]
    fn test_panel_row_width() {
        let content = Text::new("A");
        let panel = Panel::new(&content);

        // Panels must always fill 100% of the target grid width allocation
        assert_eq!(panel.row_width(0, 40), 40);
        assert_eq!(panel.row_width(1, 40), 40);
    }

    #[test]
    fn test_panel_render_borders_without_title() {
        let content = Text::new("OK");
        let panel = Panel::new(&content);
        let width = 10;

        // Row 0: Top Border
        let top = format!("{}", RowRenderer(&panel, 0, width));
        assert_eq!(top, "┌────────┐");

        // Row 1: Middle Content Row (Wrapped with "│" and padded with spaces to fill width 10)
        let middle = format!("{}", RowRenderer(&panel, 1, width));
        assert_eq!(middle, "│OK      │");

        // Row 2: Bottom Border
        let bottom = format!("{}", RowRenderer(&panel, 2, width));
        assert_eq!(bottom, "└────────┘");
    }

    #[test]
    fn test_panel_render_borders_with_title() {
        let content = Text::new("Go");
        let panel = Panel::new(&content).title("App");
        let width = 12; // content_width will be 10

        // Title padding math check:
        // content_width (10) - (visual_line_width("App") (3) + 2) = 5 remaining lines
        let top = format!("{}", RowRenderer(&panel, 0, width));
        assert_eq!(top, "┌─App──────┐");

        let middle = format!("{}", RowRenderer(&panel, 1, width));
        assert_eq!(middle, "│Go        │");
    }

    #[test]
    fn test_panel_render_handling_asymmetrical_content_padding() {
        let content = Text::new("LongerLine\nShort");
        let panel = Panel::new(&content);
        let width = 16; // content_width = 14

        // Row 1: LongerLine (Length 10 -> Padding spaces = 14 - 10 = 4)
        let middle_1 = format!("{}", RowRenderer(&panel, 1, width));
        assert_eq!(middle_1, "│LongerLine    │");

        // Row 2: Short (Length 5 -> Padding spaces = 14 - 5 = 9)
        let middle_2 = format!("{}", RowRenderer(&panel, 2, width));
        assert_eq!(middle_2, "│Short         │");
    }

    #[test]
    fn test_panel_layout_display_lazy_integration() {
        let content = Text::new("Hi");
        let panel = Panel::new(&content).title("Box");

        let display = LayoutDisplay {
            layout: &panel,
            width: 8, // content_width = 6
        };

        let output = format!("{}", display);
        let expected = "┌─Box──┐\n\
                              │Hi    │\n\
                              └──────┘";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_panel_layout_for_nested_panels() {
        let content = Text::new("HELLO");
        let panel1 = Panel::new(&content);
        let panel2 = Panel::new(&panel1);

        let display = LayoutDisplay {
            layout: &panel2,
            width: 10,
        };

        let output = format!("{}", display);
        let expected = "┌────────┐\n\
                              │┌──────┐│\n\
                              ││HELLO ││\n\
                              │└──────┘│\n\
                              └────────┘";

        assert_eq!(output, expected);
    }
}
