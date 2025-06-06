use bevy::prelude::{Event, Entity};

#[derive(Debug, Event)]
pub struct PlayerSpawnedEvent(pub Entity);

#[derive(Debug, Event)]
pub struct EnemySpawnedEvent(pub Entity);
