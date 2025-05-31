use bevy::prelude::{*, Plugin as BevyPlugin};
use bevy_ascii_terminal::{color::BLACK, *};

use crate::resource::*;
use super::{UI_SIZE, system::*, component::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (clear, (draw_player_stats, draw_game_log)).chain())
        ;
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    let ui_size = [config.map_width - 2, UI_SIZE - 2];
    let terminal = Terminal::new(ui_size);

    commands.spawn((
        terminal,
        TerminalMeshPivot::TopCenter,
        TerminalBorder::single_line(),
        UiTerminal
    ));
}

