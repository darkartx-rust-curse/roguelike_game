use bevy::prelude::{Query, With, Res};
use bevy_ascii_terminal::{border::BorderSide, *};

use crate::{
    component::{Health, MaxHelth, Player},
    resource::{GameLog}
};

use super::component::*;

pub(super) fn clear(mut terminal: Query<&mut Terminal, With<UiTerminal>>) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => return
    };

    terminal.clear();
}

pub(super) fn draw_player_stats(
    mut terminal: Query<&mut TerminalBorder, With<UiTerminal>>,
    health: Query<(&Health, &MaxHelth), With<Player>>
) {
    let mut terminal_border = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => return
    };

    terminal_border.clear_strings();

    if let Ok((health, max_health)) = health.single() {
        let hp_string = format!("HP {} / {}", health.0, max_health.0);

        terminal_border.put_string( BorderSide::Top, 1.0, 0, hp_string);
    }
}

pub(super) fn draw_game_log(
    mut terminal: Query<&mut Terminal, With<UiTerminal>>,
    game_log: Res<GameLog>
) {
    let mut terminal = match terminal.single_mut() {
        Ok(terminal) => terminal,
        _ => return
    };

    let log_max_count = terminal.height();
    let mut log_entries = game_log.entries().into_iter().rev();

    for i in 0..log_max_count {
        let log_entry = match log_entries.next() {
            Some(log_entry) => log_entry,
            _ => break
        };

        terminal.put_string([0, log_max_count - i - 1], log_entry.to_string());
    }
}
