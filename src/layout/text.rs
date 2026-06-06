use crate::layout::{LayoutDisplay, Renderable, SizeHint};
use crate::terminal::visual_line_width;
use core::{cmp, fmt};

pub struct Text<'a> {
    pub text: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> fmt::Display for Text<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // temporary display configuration proxy
        let display = LayoutDisplay {
            layout: self,
            width: 80,
        };

        fmt::Display::fmt(&display, f)
    }
}

impl<'a> Renderable for Text<'a> {
    fn measure(&self, max_width: usize) -> SizeHint {
        // Find the longest single visual line to propose a correct width hint
        let mut max_visual_len = 0;
        for line in self.text.lines() {
            max_visual_len = cmp::max(max_visual_len, visual_line_width(line));
        }

        SizeHint {
            min: cmp::min(max_visual_len, max_width),
            max: Some(cmp::min(max_visual_len, max_width)),
        }
    }

    fn total_rows(&self, _width: usize) -> usize {
        self.text.lines().count()
    }

    fn row_width(&self, row_idx: usize, _width: usize) -> usize {
        // If your text component includes prettyt ANSI sequences,
        // you would use your `visible_width()` stripping loop here.
        self.text
            .lines()
            .nth(row_idx)
            .map(visual_line_width)
            .unwrap_or(0)
    }

    fn render_row(&self, row_idx: usize, _width: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(line) = self.text.lines().nth(row_idx) {
            f.write_str(line)?;
        }
        Ok(())
    }
}
