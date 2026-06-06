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
