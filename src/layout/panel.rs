use crate::layout::{LayoutDisplay, Renderable, SizeHint};
use crate::terminal::visual_line_width;
use core::fmt;

pub struct Panel<'a, T: Renderable> {
    pub content: T,
    pub title: Option<&'a str>,
}

impl<'a, T: Renderable> Panel<'a, T> {
    /// Creates a new panel structure enclosing a piece of layout content.
    pub fn new(content: T) -> Self {
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
            width: 80,
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
