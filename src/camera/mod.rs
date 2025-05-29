use bevy::prelude::*;
use engine::player::PlayerComponent;
use crate::consts::{CAMERA_HEIGHT, SMOOTHING};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(PostUpdate, camera_follow_player);
    }
}

#[derive(Component)]
pub struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<PlayerComponent>)>,
    player_query: Query<&Transform, (With<PlayerComponent>, Without<GameCamera>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };


    let player_position = player_transform.translation.truncate();
    let camera_position = camera_transform.translation.truncate();
    let new_camera_position = camera_position.lerp(player_position, SMOOTHING);

    camera_transform.translation = new_camera_position.extend(CAMERA_HEIGHT);
}
