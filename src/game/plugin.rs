use bevy::prelude::{Plugin as BevyPlugin, App, Startup, Update};

use super::system::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_player, spawn_enemies))
            .add_systems(Update, left_walker)
        ;
    }
}