use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::base::{Size, Velocity};
use crate::core::domain::entity::entity::Ai;
use bevy::audio::AudioLoader;
use bevy::audio::AudioSource;
use bevy::log;
use bevy::prelude::{
    AssetServer, AudioPlayer, Commands, PlaybackSettings, Query, Res, Transform, Window, With,
};
use bevy::time::Time;
use bevy::window::PrimaryWindow;
use rand::random;

pub fn ai_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Velocity), (With<Ai>, With<Enemy>)>,
    time: Res<Time>,
) {
    for (mut transform, direction) in enemy_query.iter_mut() {
        transform.translation += time.delta_secs() + direction.0;
    }
}

pub fn ai_enemy_confine_movement(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Transform, &mut Velocity, &Size), (With<Ai>, With<Enemy>)>,
) {
    let Ok(window) = window_query.get_single() else {
        log::error!("Failed to get window");
        return;
    };

    for (mut transform, mut direction, size) in enemy_query.iter_mut() {
        let half_size_x = size.0.x / 2.0;
        let half_size_y = size.0.y / 2.0;

        let x_min = half_size_x;
        let x_max = window.width() - half_size_x;
        let y_min = half_size_y;
        let y_max = window.height() - half_size_y;

        let mut bounced = false;

        if transform.translation.x <= x_min {
            transform.translation.x = x_min;
            direction.0.x = -direction.0.x;
            bounced = true;
        } else if transform.translation.x >= x_max {
            transform.translation.x = x_max;
            direction.0.x = -direction.0.x;
            bounced = true;
        }

        if transform.translation.y <= y_min {
            transform.translation.y = y_min;
            direction.0.y = -direction.0.y;
            bounced = true;
        } else if transform.translation.y >= y_max {
            transform.translation.y = y_max;
            direction.0.y = -direction.0.y;
            bounced = true;
        }

        if bounced {
            direction.0 = direction.0.normalize();
            let bounce_effect = asset_server.load("audio/pluck_001.ogg");

            commands.spawn((AudioPlayer::new(bounce_effect), PlaybackSettings::default()));
        }
    }
}
