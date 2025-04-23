use bevy::prelude::*;

mod asset;
mod camera;
mod game;
mod input;
mod map;
mod player;
mod state;
mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)))
        .add_plugins(DefaultPlugins)
        .run();
}
