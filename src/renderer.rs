use bevy::prelude::{*, Plugin as BevyPlugin};
use bevy_ascii_terminal::{TerminalPlugins, TerminalCamera};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TerminalPlugins)
            .add_systems(Startup, setup)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCamera::new());
}
