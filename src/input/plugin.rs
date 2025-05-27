use bevy::prelude::{Plugin as BevyPlugin, App, Update};

use super::system::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_input)
        ;
    }
}