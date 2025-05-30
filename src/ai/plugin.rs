use bevy::prelude::{*, Plugin as BevyPlugin};

use crate::resource::*;
use super::system::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(TurnState::EnemyTurn), enemy_ai)
        ;
    }
}
