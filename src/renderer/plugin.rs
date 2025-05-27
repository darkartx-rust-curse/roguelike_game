use bevy::{ecs::schedule::IntoScheduleConfigs, prelude::{App, Commands, Plugin as BevyPlugin, Startup, Update}};
use bevy_ascii_terminal::{color::BLACK, *};

use super::system::*;

const TERMINAL_SIZE: [i32; 2] = [80, 50];

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TerminalPlugins)
            .add_systems(Startup, setup)
            .add_systems(Update, (clear, render_player, render_enemies).chain())
        ;
    }
}

fn setup(mut commands: Commands) {
    let clear_tile = Tile::new(' ', BLACK, BLACK);

    commands.spawn(Terminal::new(TERMINAL_SIZE).with_clear_tile(clear_tile));
    commands.spawn(TerminalCamera::new());
}

