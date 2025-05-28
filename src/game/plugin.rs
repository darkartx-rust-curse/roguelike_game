use bevy::prelude::{*, Plugin as BevyPlugin};

use crate::{resource::*, event::*};
use super::{resource::*, system::*};

// Минимальное время цикла одного хода
const TURN_MIN_TIME_SECS: f32 = 0.1;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DiceBox::default())
            .insert_resource(TurnTimer::new(TURN_MIN_TIME_SECS))
            .init_state::<TurnState>()
            .add_event::<PlayerSpawnedEvent>()
            .add_event::<EnemySpawnedEvent>()
            .add_systems(Startup, (generate_map, (spawn_player, spawn_enemies, turn_start)).chain())
            .add_systems(Update, (turn_delay).run_if(in_state(TurnState::EndTurn)))
        ;
    }
}