use bevy::prelude::{Timer, Resource};

#[derive(Debug, Resource)]
pub struct InputDelayTimer(pub Timer);
