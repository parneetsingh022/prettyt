pub mod detect;
pub mod registry;

pub use detect::{ColorLevel, detect_color_level};
pub(crate) use registry::get_cached_level;
