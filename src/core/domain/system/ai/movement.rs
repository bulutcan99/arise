use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::movement::Direction;
use crate::core::domain::entity::entity::Ai;
use crate::core::domain::system::ai::common::{AI_ENEMY_SIZE, AI_ENEMY_SPEED};
use bevy::log;
use bevy::prelude::{Query, Res, Transform, Window, With};
use bevy::time::Time;
use bevy::window::PrimaryWindow;

pub fn ai_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Direction), (With<Ai>, With<Enemy>)>,
    time: Res<Time>,
) {
    for (mut transform, direction) in enemy_query.iter_mut() {
        transform.translation += time.delta_secs() + direction.0 * AI_ENEMY_SPEED;
    }
}

pub fn ai_enemy_bounce(
    mut enemy_query: Query<(&mut Transform, &mut Direction), (With<Ai>, With<Enemy>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        log::error!("Failed to get window");
        return;
    };

    let half_size = AI_ENEMY_SIZE / 2.0;

    let x_min = half_size;
    let x_max = window.width() - half_size;
    let y_min = half_size;
    let y_max = window.height() - half_size;

    for (mut transform, mut direction) in enemy_query.iter_mut() {
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
        }
    }
}
