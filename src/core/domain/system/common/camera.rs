use crate::core::domain::entity::entity::Player;
use bevy::prelude::{Camera, Camera2d, Query, Transform, With, Without};

pub fn follow_player(
    player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok(mut player_transform) = player_query.get_single() {
        if let Ok(mut camera) = camera_query.get_single_mut() {
            camera.translation.x = player_transform.translation.x;
            camera.translation.y = player_transform.translation.y;
        }
    }
}
