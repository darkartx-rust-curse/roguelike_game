use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{
    component::{Player, Position, Enemy, Name},
    map::{Map, Viewshed, RevealedMap}
};

use super::{component::*, constants::*};

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

pub(super) fn cursor_position(
    mut cursor: Query<(&mut Cursor, &TerminalTransform), With<ViewportTerminal>>,
    camera: Query<&TerminalCamera>
) {
    let Ok(camera) = camera.single() else {
        return
    };

    let Ok((mut cursor, transform)) = cursor.single_mut() else {
        return
    };

    let Some(cursor_position) = camera.cursor_world_pos() else {
        return
    };

    if let Some(cursor_position) = transform.world_to_tile(cursor_position) {
        cursor.show_at(cursor_position.as_uvec2());
    } else {
        cursor.hide();
    }
}

pub(super) fn draw_cursor(
    mut terminal: Query<(&mut Terminal, &Cursor), With<ViewportTerminal>>,
) {
    let Ok((mut terminal, cursor)) = terminal.single_mut() else {
        return
    };

    if !cursor.show {
        return
    }

    terminal.put_bg_color(cursor.position, CURSOR_COLOR);
}

pub(super) fn draw_tooltip(
    mut terminal: Query<(&mut Terminal, &Cursor), With<ViewportTerminal>>,
    entities: Query<(&Name, &Position)>,
    player_viewshed: Query<&Viewshed, With<Player>>
) {
    let Ok((mut terminal, cursor)) = terminal.single_mut() else {
        return
    };

    if !cursor.show {
        return
    }

    let Ok(player_viewshed) = player_viewshed.single() else {
        return
    };

    let mut tooltip: Vec<&String> = Vec::new();

    for (name, position) in entities {
        if !player_viewshed.visible_tiles().contains(&position.0) ||
            cursor.position != position.0 {
            continue;
        }

        tooltip.push(&name.0);
    }

    if tooltip.is_empty() {
        return
    }

    let width = tooltip.iter().map(|i| i.len()).max().unwrap() as u32 + 3;

    let is_left = cursor.position.x > width + 5;

    let x = if is_left {
        cursor.position.x - width
    } else {
        cursor.position.x + 1
    };
    let mut y = cursor.position.y;
    let mut first = true;

    for s in tooltip {
        let s = if first {
            first = false;
            if is_left {
                format!("{s} ->")
            } else {
                format!("<- {s}")
            }
        } else {
            if is_left {
                format!("{s}   ")
            } else {
                format!("   {s}")
            }
        };

        terminal.put_string(
            [x, y].pivot(Pivot::BottomLeft),
            s.bg(TOOLTIP_BG).fg(TOOLTIP_FG)
        );

        y += 1;
    }
}


