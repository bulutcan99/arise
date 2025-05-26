use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use options::display::DisplayConfig;

mod animation;
mod camera;
mod consts;
mod development;
mod game;
mod options;
mod player;
mod plugins;
mod states;
mod ui;

fn main() {}

// TODO: After some time player will set display!
fn set_display_config() -> DisplayConfig {
    DisplayConfig::new()
}

fn default_plugins(display: DisplayConfig) -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window::from(display)),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
}
// ()
// fn custom_plugins() -> PluginGroupBuilder {}
//
// fn build_app<P1: PluginGroup, P2: PluginGroup>(
// default_plugins: P1,
// custom_plugins: P2,
// ) -> App {
// }
