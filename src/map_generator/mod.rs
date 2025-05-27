pub mod noisy_generator;
pub mod dungeon_generator;

use crate::map::Map;

pub use noisy_generator::*;
pub use dungeon_generator::*;

pub trait Generator {
    fn generate(&mut self) -> Map;
}