use bevy::prelude::{*, Plugin as BevyPlugin};

use super::system::*;
use crate::{resource::*, event::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DiceBox::default())
            .add_event::<PlayerSpawnedEvent>()
            .add_systems(Startup, (generate_map, (spawn_player)).chain())
        ;
    }
}