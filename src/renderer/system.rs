use bevy::prelude::Query;
use bevy_ascii_terminal::Terminal;

use crate::component::{Player, Enemy, Position};

use super::component::*;

pub(super) fn clear(mut terminal: Query<&mut Terminal>) {
    let mut terminal = terminal.single_mut().unwrap();

    terminal.clear();
}

pub(super) fn render_player(mut terminal: Query<&mut Terminal>, player: Query<(&Player, &Position)>) {
    let mut terminal = terminal.single_mut().unwrap();

    for (player, position) in player {
        let tile = player.to_tile();

        terminal.put_tile(position.0, tile);
    }
}

pub(super) fn render_enemies(mut terminal: Query<&mut Terminal>, enemies: Query<(&Enemy, &Position)>) {
    let mut terminal = terminal.single_mut().unwrap();

    for (enemy, position) in enemies {
        let tile = enemy.to_tile();

        terminal.put_tile(position.0, tile);
    }
}