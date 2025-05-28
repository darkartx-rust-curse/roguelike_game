use bevy::prelude::{*, Plugin as BevyPlugin};

use crate::resource::*;
use super::system::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (enemy_ai).run_if(in_state(TurnState::EnemyTurn)))
        ;
    }
}
