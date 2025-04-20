use crate::core::domain::component::attribute::{Dexterity, Intelligence, Strength};
use crate::core::domain::component::combat::{AttackPower, Defense};
use crate::core::domain::component::hunter_type::Warrior;
use crate::core::domain::component::stat::{Experience, Health, Level, Mana, Stamina};
use bevy::asset::AssetServer;
use bevy::prelude::{
    Camera, Camera2d, Commands, GlobalTransform, Query, Res, Sprite, Transform, Vec2, Vec3, Window,
    With,
};
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = query.get_single() else {
        bevy::log::error!("Failed to get primary window handle");
        panic!("Failed to get primary window handle");
    };

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        GlobalTransform::default(),
    ));
}
