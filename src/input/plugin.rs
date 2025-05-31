use bevy::prelude::{*, Plugin as BevyPlugin};

use crate::resource::TurnState;
use super::{INPUT_DELAY_SECS, system::*, resource::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                InputDelayTimer(Timer::from_seconds(INPUT_DELAY_SECS, TimerMode::Repeating))
            )
            .add_systems(
                Update, 
                player_input.run_if(in_state(TurnState::PlayerTurn))
            )
        ;
    }
}