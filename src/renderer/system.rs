use bevy::prelude::{Query, With};
use bevy_ascii_terminal::Terminal;

use crate::{
    component::{Player, Position},
    map::{Map, Viewshed, RevealedMap}
};

use super::tile::*;

pub(super) fn clear(mut terminal: Query<&mut Terminal>) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => { return }
    };

    terminal.clear();
}

pub(super) fn render_map(
    mut terminal: Query<&mut Terminal>,
    map: Query<&Map>,
    player_map: Query<(&Viewshed, Option<&RevealedMap>), With<Player>>
) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => { return }
    };
    let map = match map.single() {
        Ok(map) => map,
        _ => { return }
    };
    let (player_viewshed, player_revealed_map) = match player_map.single() {
        Ok(player_map) => player_map,
        _ => { return }
    };

    for x in 0..map.size().x {
        for y in 0..map.size().y {
            let position = (x, y).into();
            let visible = player_viewshed.visible_tiles().contains(&position);

            let map_tile = if visible {
                Some(map.tile(position))
            } else {
                player_revealed_map.map(|map| map.tile(position))
            };

            let tile = map_tile.map(|map_tile| map_tile.to_tile(visible));
            
            if let Some(tile) = tile {
                terminal.put_tile(position, tile);
            }
        }
    }
}

pub(super) fn render_player(mut terminal: Query<&mut Terminal>, player: Query<(&Player, &Position)>) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => { return }
    };

    for (player, position) in player {
        let tile = player.to_tile(true);

        terminal.put_tile(position.0, tile);
    }
}