use crate::core::domain::system::common::camera::follow_player;
use crate::core::domain::system::common::command::spawn_camera;
use crate::core::domain::system::player::command::spawn_player;
use crate::core::domain::system::player::movement::player_movement;
use bevy::prelude::*;

mod core;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .run();
    Ok(())
}
