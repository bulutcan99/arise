use background::generator::BackgroundPlugin;
use bevy::prelude::*;
use bevy_tiling_background::{BackgroundMaterial, TilingBackgroundPlugin};

mod background;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilingBackgroundPlugin::<BackgroundMaterial>::default())
        .add_plugins(BackgroundPlugin)
        .run();
}
