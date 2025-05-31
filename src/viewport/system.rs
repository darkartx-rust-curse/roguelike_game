use bevy::prelude::{Query, With};
use bevy_ascii_terminal::Terminal;

use crate::{
    component::{Player, Position, Enemy},
    map::{Map, Viewshed, RevealedMap}
};

use super::component::*;

pub(super) fn clear(mut terminal: Query<&mut Terminal, With<ViewportTerminal>>) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => { return }
    };

    terminal.clear();
}

pub(super) fn draw_map(
    mut terminal: Query<&mut Terminal, With<ViewportTerminal>>,
    map: Query<&Map>,
    player_map: Query<(&Viewshed, &RevealedMap), With<Player>>
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
                map.tile(position)
            } else {
                player_revealed_map.tile(position)
            };

            terminal.put_tile(position, map_tile.to_tile(visible));
        }
    }
}

pub(super) fn draw_player(
    mut terminal: Query<&mut Terminal, With<ViewportTerminal>>,
    player: Query<(&Player, &Position)>
) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => { return }
    };

    for (player, position) in player {
        let tile = player.to_tile(true);

        terminal.put_tile(position.0, tile);
    }
}

// Рендерим монстров только тех, которых видим
pub(super) fn draw_enemies(
    mut terminal: Query<&mut Terminal, With<ViewportTerminal>>,
    enemies: Query<(&Enemy, &Position)>,
    player_viewshed: Query<&Viewshed, With<Player>>
) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => { return }
    };

    let player_viewshed = match player_viewshed.single() {
        Ok(player_viewshed) => player_viewshed,
        _ => return
    };

    let enemies = enemies.iter()
        .filter(|(_, position)| player_viewshed.visible_tiles().contains(&position.0));

    for (enemy, position) in enemies {
        let tile = enemy.to_tile(true);

        terminal.put_tile(position.0, tile);
    }
}
