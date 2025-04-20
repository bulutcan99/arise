use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::base::{Size, Velocity};
use crate::core::domain::component::combat::AttackPower;
use crate::core::domain::component::hunter_type::Warrior;
use crate::core::domain::component::stat::{Experience, Health, Level};
use crate::core::domain::entity::entity::{Ai, Player};
use crate::core::domain::system::ai::common::AI_ENEMY_NUMBER;
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    Commands, Query, Res, Sprite, Transform, Vec2Swizzles, Vec3Swizzles, Window, With,
};
use bevy::window::PrimaryWindow;
use rand::{random, random_range};

pub fn spawn_ai_enemy(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(window) = query.get_single() else {
        bevy::log::error!("Failed to get primary window handle");
        panic!("Failed to get primary window handle");
    };

    let texture = asset_server.load("sprites/ball_red_large.png");
    for id in 0..AI_ENEMY_NUMBER {
        bevy::log::info!("Creating enemy{id}");
        let rand_width = random::<f32>() * window.width();
        let rand_height = random::<f32>() * window.height();
        let random_size = random_range(40.0..=60.0);
        let ball_size = Vec3::new(random_size, random_size, 0.0);
        commands.spawn((
            Ai,
            Enemy,
            Health(50),
            AttackPower(20),
            Velocity(Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize()),
            Size(ball_size),
            Transform::from_xyz(rand_width, rand_height, 0.0),
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec3::xy(ball_size)),
                ..Default::default()
            },
        ));
    }
}
