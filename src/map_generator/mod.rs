pub mod noisy_generator;

pub use noisy_generator::*;

use crate::map::Map;

pub trait Generator {
    fn generate(&mut self) -> Map;
}