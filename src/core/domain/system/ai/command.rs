use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::class::Warrior;
use crate::core::domain::component::combat::AttackPower;
use crate::core::domain::component::stat::{Experience, Health, Level};
use crate::core::domain::entity::player::Player;
use crate::core::domain::system::ai::common::ENEMY_NUMBER;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, Sprite, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::random;

pub fn spawn_enemy(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(window) = query.get_single() else {
        bevy::log::error!("Failed to get primary window handle");
        panic!("Failed to get primary window handle");
    };

    let texture = asset_server.load("sprites/ball_red_large.png");
    for id in 0..ENEMY_NUMBER {
        bevy::log::info!("Creating enemy{id}");
        //TODO: collassion hesaplamasi
        let rand_width = random::<f32>() * window.width();
        let rand_height = random::<f32>() * window.height();

        commands.spawn((
            Enemy,
            Health(50),
            AttackPower(20),
            Sprite {
                image: texture.clone(),
                ..Default::default()
            },
            Transform::from_xyz(rand_width, rand_height, 0.0),
        ));
    }
}
