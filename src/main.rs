use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use settings::display::DisplayConfig;

mod animation;
mod camera;
mod consts;
mod development;
mod player;
mod settings;
mod states;

fn main() {}

// TODO: After some time player will set display!
fn set_display_config() -> DisplayConfig {
    DisplayConfig::new();
}

fn default_plugins(display: DisplayConfig) -> PluginGroupBuilder {}

fn custom_plugins() -> PluginGroupBuilder {}

fn build_app<P1: PluginGroup, P2: PluginGroup>(
    default_plugins: P1,
    custom_plugins: P2,
) -> App {
}
