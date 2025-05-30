use bevy::prelude::{*, Plugin as BevyPlugin};

use crate::{resource::*, event::*};
use super::{TURN_MIN_TIME_SECS, resource::*, system::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DiceBox::default())
            .insert_resource(TurnTimer::new(TURN_MIN_TIME_SECS))
            .init_state::<TurnState>()
            .add_event::<PlayerSpawnedEvent>()
            .add_event::<EnemySpawnedEvent>()
            .add_systems(Startup, (generate_map, (spawn_player, spawn_enemies, start_turn)).chain())
            .add_systems(Update, turn_system)
            .add_systems(OnExit(TurnState::PlayerTurn), player_movement)
        ;
    }
}