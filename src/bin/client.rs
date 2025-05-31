use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use roguelike_game::*;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dotenv::dotenv().unwrap();

    let window_title = format!("{} ({})", APP_NAME, APP_VERSION);

    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            title: window_title,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                }
            )
        )
        .add_plugins((ScreenDiagnosticsPlugin::default(), ScreenFrameDiagnosticsPlugin))
        .add_plugins(ClientPlugins)
        .run()
    ;
}