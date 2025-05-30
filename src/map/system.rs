use bevy::prelude::*;

use crate::component::{BlocksTile, Position};
use super::{Map, Viewshed, RevealedMap};

pub(super) fn visablity_system(viewsheds: Query<(&Position, &mut Viewshed, Option<&mut RevealedMap>)>, map: Query<&Map>) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return
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

pub(super) fn update_blocking(
    mut map: Query<&mut Map>,
    blocks_tiles: Query<&Position, With<BlocksTile>>
) {
    let mut map = match map.single_mut() {
        Ok(map) => map,
        _ => return
    };

    map.populate_blocking();

    for position in blocks_tiles {
        map.set_blocked(position.0, true);
    }
}
