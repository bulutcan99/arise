use crate::core::domain::component::attribute::{Dexterity, Intelligence, Strength};
use crate::core::domain::component::class::Warrior;
use crate::core::domain::component::combat::{AttackPower, Defense};
use crate::core::domain::component::stat::{Experience, Health, Level, Mana, Stamina};
use crate::core::domain::entity::player::Player;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, Sprite, Transform, Vec2, Vec3, Window, With};
use bevy::window::PrimaryWindow;

pub fn spawn_player(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(window) = query.get_single() else {
        bevy::log::error!("Failed to get primary window handle");
        panic!("Failed to get primary window handle");
    };

    let texture = asset_server.load("sprites/ball_blue_large.png");

    bevy::log::info!("Creating player");
    commands.spawn((
        Player,
        Warrior,
        Level(1),
        Experience(0),
        Health(100),
        AttackPower(50),
        Sprite {
            image: texture.clone(),
            ..Default::default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}
