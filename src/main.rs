use bevy::prelude::*;

mod camera;
mod game;
mod input;
mod map;
mod player;
mod states;
mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
