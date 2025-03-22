use crate::core::domain::component::attribute::{Dexterity, Intelligence, Strength};
use crate::core::domain::component::base::{Size, Velocity};
use crate::core::domain::component::class::Warrior;
use crate::core::domain::component::combat::{AttackPower, Defense};
use crate::core::domain::component::stat::{Experience, Health, Level, Mana, Stamina};
use crate::core::domain::entity::entity::Player;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Query, Res, Sprite, Transform, Vec2, Vec3, Vec3Swizzles, Window, With,
};
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
    let player_size = Vec3::new(50.0, 50.0, 0.0);
    commands.spawn((
        Player,
        Warrior,
        Level(1),
        Experience(0),
        Health(100),
        AttackPower(50),
        Velocity(Vec3::ZERO),
        Size(player_size),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Sprite {
            image: texture.clone(),
            custom_size: Some(Vec3::xy(player_size)),
            ..Default::default()
        },
    ));
}
