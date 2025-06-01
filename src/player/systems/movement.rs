use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use engine::input::PlayerAction;
use engine::player::PlayerMobilityComponent;
use leafwing_input_manager::prelude::ActionState;

use crate::game::resources::GameResource;

pub fn movement_system(
    game_parameters: Res<GameResource>,
    mut player_query: Query<(
        &PlayerMobilityComponent,
        &mut Velocity,
        &mut Transform,
        &ActionState<PlayerAction>,
    )>,
) {
    for (player_mobility, mut vel, mut transform, action_state) in
        player_query.iter_mut()
    {
        // Input okuma
        let up = action_state.pressed(&PlayerAction::MoveUp);
        let down = action_state.pressed(&PlayerAction::MoveDown);
        let left = action_state.pressed(&PlayerAction::MoveLeft);
        let right = action_state.pressed(&PlayerAction::MoveRight);

        let x_axis = match (left, right) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        let y_axis = match (down, up) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        let desired_scale_x = match x_axis {
            1 => game_parameters.sprite_scale.abs(),
            -1 => -game_parameters.sprite_scale.abs(),
            _ => transform.scale.x,
        };

        if transform.scale.x != desired_scale_x {
            transform.scale.x = desired_scale_x;
        }

        apply_axis_movement(
            x_axis,
            &mut vel.linvel.x,
            player_mobility.acceleration.x,
            player_mobility.deceleration.x,
            player_mobility.speed.x,
            game_parameters.stop_threshold,
        );

        apply_axis_movement(
            y_axis,
            &mut vel.linvel.y,
            player_mobility.acceleration.y,
            player_mobility.deceleration.y,
            player_mobility.speed.y,
            game_parameters.stop_threshold,
        );
    }
}

fn apply_axis_movement(
    axis_input: i8,
    velocity: &mut f32,
    acceleration: f32,
    deceleration: f32,
    max_speed: f32,
    stop_threshold: f32,
) {
    if axis_input != 0 {
        *velocity += acceleration * axis_input as f32;
        if velocity.abs() > max_speed {
            *velocity = velocity.signum() * max_speed;
        }
    } else if velocity.abs() > stop_threshold {
        *velocity -= deceleration * velocity.signum();
    } else {
        *velocity = 0.0;
    }
}
