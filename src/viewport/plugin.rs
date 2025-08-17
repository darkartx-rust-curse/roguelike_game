use bevy::prelude::{*, Plugin as BevyPlugin};
use bevy_ascii_terminal::{color::BLACK, *};

use crate::resource::*;
use super::{system::*, component::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(
                Update, 
                (
                    cursor_position,
                    (clear, draw_map, draw_enemies, draw_player, draw_cursor, draw_tooltip).chain()
                )
            )
        ;
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    let clear_tile = Tile::new(' ', BLACK, BLACK);

    let viewpoer_size = [config.map_width, config.map_height];
    let terminal = Terminal::new(viewpoer_size)
        .with_clear_tile(clear_tile);

    let cursor = Cursor::default();

    commands.spawn((terminal, TerminalMeshPivot::BottomCenter, cursor, ViewportTerminal));
}

