use terminal_size::{Height, Width, terminal_size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TerminalDimension {
    pub width: u16,
    pub height: u16,
}

pub(crate) fn get_terminal_size() -> Option<TerminalDimension> {
    terminal_size().map(|(Width(w), Height(h))| TerminalDimension {
        width: w,
        height: h,
    })
}
