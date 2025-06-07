use std::panic;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use engine::player::{
    PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent,
};
use engine::weapon::WeaponComponent;
use leafwing_input_manager::prelude::MouseMove;

pub fn update_weapon_transform(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut weapon_query: Query<
        &mut Transform,
        (
            With<WeaponComponent>,
            Without<PlayerComponent>,
        ),
    >,
    player_query: Query<&Transform, With<PlayerComponent>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        warn!("Weapon or player is empty");
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
        // TODO: burda karakteri ortalayacak sekilde x ve y ekseninde ayni oranda
        // mesafe birakmaliyiz
        if let Ok(ray) =
            camera.viewport_to_world(camera_transform, cursor_screen_pos)
        {
            let cursor_pos = ray.origin;
            let player_transform = player_query.single();
            let mut weapon_transform = weapon_query.single_mut();

            let player_pos = player_transform.translation;
            let direction = (cursor_pos - player_pos).normalize_or_zero();
            let distance_from_player = 100.0;

            let weapon_pos = player_pos + direction * distance_from_player;
            weapon_transform.translation = Vec3::new(
                weapon_pos.x,
                weapon_pos.y,
                player_pos.z + 0.1,
            );
            weapon_transform.rotation =
                Quat::from_rotation_z(direction.y.atan2(direction.x));
        }
    }
}
