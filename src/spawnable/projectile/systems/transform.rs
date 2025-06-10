use bevy::input::mouse::MouseButtonInput;
use bevy::input::{ButtonInput, ButtonState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use engine::player::PlayerComponent;
use engine::spawnable::projectile::ProjectileComponent;

pub fn update_projectile_transform(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut projectile_query: Query<
        &mut Transform,
        (
            With<ProjectileComponent>,
            Without<PlayerComponent>,
        ),
    >,
    player_query: Query<&Transform, With<PlayerComponent>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if player_query.is_empty() || projectile_query.is_empty() {
        warn!("Projectile or player entity missing");
        return;
    }

    // Yalnızca mouse sol tık basılıysa çalış
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = match windows_query.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(c) => c,
        Err(_) => return,
    };

    if let Some(cursor_screen_pos) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_screen_pos) {
            let cursor_pos = ray.origin;
            let player_transform = player_query.single();
            let player_pos = player_transform.translation;

            let direction = (cursor_pos - player_pos).normalize_or_zero();
            let distance_from_player = 300.0;
            let new_position = player_pos + direction * distance_from_player;

            for mut transform in projectile_query.iter_mut() {
                transform.translation = Vec3::new(
                    new_position.x,
                    new_position.y,
                    player_pos.z + 0.1,
                );
                transform.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x));
            }
        }
    }
}