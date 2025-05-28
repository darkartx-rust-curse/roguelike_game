use bevy::prelude::{Query, EventReader, Commands};

use crate::{
    component::Position,
    event::PlayerSpawnedEvent,
};
use super::{Map, Viewshed, RevealedMap};

pub(super) fn visablity_system(viewsheds: Query<(&Position, &mut Viewshed, Option<&mut RevealedMap>)>, map: Query<&Map>) {
    let map = match map.single() {
        Ok(map) => map,
        _ => { return }
    };

    for (position, mut viewshed, revealed_map) in viewsheds {
        viewshed.recalculate(map, position.0);

        if let Some(mut revealed_map) = revealed_map {
            for visible_tile in viewshed.visible_tiles() {
                revealed_map.set_tile(*visible_tile, map.tile(*visible_tile));
            }
        }
    }
}

pub(super) fn setup_entity(
    mut commands: Commands,
    mut player_spawned_event: EventReader<PlayerSpawnedEvent>,
    map: Query<&Map>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => { return }
    };

    for event in player_spawned_event.read() {
        let player = event.0;
        commands.entity(player).insert(map.make_revealed_map());
    }
}
