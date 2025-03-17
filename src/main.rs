use crate::core::domain::system::ai::command::spawn_ai_enemy;
use crate::core::domain::system::ai::movement::{ai_enemy_bounce, ai_enemy_movement};
use crate::core::domain::system::common::command::spawn_camera;
use crate::core::domain::system::player::command::spawn_player;
use crate::core::domain::system::player::movement::{confine_player_movement, player_movement};
use bevy::prelude::*;

mod core;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_ai_enemy)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, ai_enemy_movement)
        .add_systems(Update, ai_enemy_bounce)
        .run();
    Ok(())
}
