use crate::core::domain::entity::entity::Player;
use crate::core::domain::system::player::common::{PLAYER_SIZE, PLAYER_SPEED};
use bevy::log;
use bevy::prelude::{ButtonInput, Camera, KeyCode, Query, Res, Time, Transform, Vec3, With};
use bevy::window::{PrimaryWindow, Window};

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        log::error!("Failed to get player's movement");
        return;
    };

    let mut direction = Vec3::ZERO;
    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::ArrowLeft => {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            KeyCode::ArrowRight => {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            KeyCode::ArrowUp => {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            KeyCode::ArrowDown => {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
            _ => {}
        }
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    player_transform.translation += time.delta_secs() + direction * PLAYER_SPEED;
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let Ok(window_query) = window_query.get_single() else {
            log::error!("Failed to get window");
            return;
        };

        let half_size_player = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_size_player;
        let x_max = window_query.width() - half_size_player;
        let y_min = 0.0 + half_size_player;
        let y_max = window_query.height() - half_size_player;

        player_transform.translation.x = player_transform.translation.x.clamp(x_min, x_max);
        player_transform.translation.y = player_transform.translation.y.clamp(y_min, y_max);
    }
}
