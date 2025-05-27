use std::ops;

use bevy::prelude::Resource;

use bracket_random::prelude::*;

#[derive(Resource)]
pub struct DiceBox {
    inner: RandomNumberGenerator
}

impl Default for DiceBox {
    fn default() -> Self {
        Self { inner: RandomNumberGenerator::new() }
    }
}

impl ops::Deref for DiceBox {
    type Target = RandomNumberGenerator;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for DiceBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
