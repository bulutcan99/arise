use crate::core::domain::entity::player::Player;
use bevy::log;
use bevy::prelude::{ButtonInput, KeyCode, Query, Res, Time, Transform, Vec3, With};

const PLAYER_SPEED: f32 = 10.0;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        bevy::log::error!("Failed to get player's movement");
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
