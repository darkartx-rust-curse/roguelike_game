pub mod plugin;
pub mod component;
mod system;
mod resource;

pub use plugin::*;

// Рейнж видимости
const VIEWSHED_RANGE: u32 = 8;

// Минимальное время цикла одного хода
const TURN_MIN_TIME_SECS: f32 = 0.15;
