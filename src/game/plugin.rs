use bevy::prelude::{Plugin as BevyPlugin, App, Startup, Update};

use super::system::*;
use crate::resource::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DiceBox::default())
            .add_systems(Startup, (generate_map, spawn_player, /* spawn_enemies */))
            .add_systems(Update, left_walker)
        ;
    }
}