use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use options::display::DisplayConfig;

mod animation;
mod camera;
mod consts;
mod dev;
mod game;
mod options;
mod player;
mod plugins;
mod states;
mod ui;
mod combat;
mod run;
mod scanner;
mod spawnable;

fn main() {}

// TODO: Player will set display!
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
