use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::base::Velocity;
use crate::core::domain::component::class::Warrior;
use crate::core::domain::component::combat::AttackPower;
use crate::core::domain::component::stat::{Experience, Health, Level};
use crate::core::domain::entity::entity::{Ai, Player};
use crate::core::domain::system::ai::common::AI_ENEMY_NUMBER;
use crate::core::domain::system::common::common::BALL_SIZE;
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Query, Res, Sprite, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;

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

        commands.spawn((
            Ai,
            Enemy,
            Health(50),
            AttackPower(20),
            Velocity(Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize()),
            Transform::from_xyz(rand_width, rand_height, 0.0),
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(BALL_SIZE / 2.0, BALL_SIZE / 2.0)),
                ..Default::default()
            },
        ));
    }
}
