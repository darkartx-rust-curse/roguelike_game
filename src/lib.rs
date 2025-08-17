pub mod viewport;
pub mod input;
pub mod component;
pub mod map;
pub mod random;
pub mod resource;
pub mod event;
pub mod ai;
pub mod plugin;
pub mod ui;
pub mod renderer;
mod system;
mod constants;
mod spawner;
mod inventory_system;

use bevy::prelude::{App, Plugin as BevyPlugin};

pub use viewport::Plugin as ViewportPlugin;
pub use input::Plugin as InputPlugin;
pub use map::Plugin as MapPlugin;
pub use ai::Plugin as AiPlugin;
pub use plugin::Plugin as GamePlugin;
pub use ui::Plugin as UiPlugin;
pub use renderer::Plugin as RendererPlugin;

pub struct ClientPlugins;

impl BevyPlugin for ClientPlugins {
    fn build(&self, app: &mut App) {
        app.
            add_plugins((
                GamePlugin,
                RendererPlugin,
                ViewportPlugin,
                UiPlugin,
                InputPlugin,
                MapPlugin,
                AiPlugin
            ))
        ;
    }
}
