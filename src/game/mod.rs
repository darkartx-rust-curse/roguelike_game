pub mod plugin;
pub mod component;
mod system;
mod resource;

pub use plugin::*;

const VIEWSHED_RANGE: u32 = 8;
