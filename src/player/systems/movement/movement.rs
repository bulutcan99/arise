use crate::game::resources::GameResource;
use bevy::prelude::*;
use engine::events::MoveEvent;
use engine::player::{
    PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent,
};
use engine::states::player::PlayerState;

pub fn movement_system(
    time: Res<Time>,
    game_parameters: Res<GameResource>,
    mut movement_events: EventReader<MoveEvent>,
    mut player_query: Query<
        (
            &PlayerMobilityComponent,
            &mut Transform,
            &PlayerState,
            &mut PlayerVelocityComponent,
        ),
        With<PlayerComponent>,
    >,
) {
    for event in movement_events.read() {
        let direction = event.direction;

        // event.entity ile eşleşen entity varsa hareket uygula
        if let Ok((
            player_mobility,
            mut transform,
            _current_animation_state,
            mut player_velocity,
        )) = player_query.get_mut(event.entity)
        {
            let x_axis = direction.x.round() as i8;
            let y_axis = direction.y.round() as i8;

            // Sprite yönü (flip)
            let desired_scale_x = match x_axis {
                1 => game_parameters.sprite_scale.abs(),
                -1 => -game_parameters.sprite_scale.abs(),
                _ => transform.scale.x,
            };

            if (transform.scale.x - desired_scale_x).abs() > f32::EPSILON {
                transform.scale.x = desired_scale_x;
            }

            apply_axis_movement(
                x_axis,
                &mut player_velocity.0,
                player_mobility.acceleration.x,
                player_mobility.deceleration.x,
                player_mobility.speed.x,
                game_parameters.stop_threshold,
            );

            apply_axis_movement(
                y_axis,
                &mut player_velocity.1,
                player_mobility.acceleration.y,
                player_mobility.deceleration.y,
                player_mobility.speed.y,
                game_parameters.stop_threshold,
            );

            transform.translation.x += player_velocity.0 * time.delta_secs();
            transform.translation.y += player_velocity.1 * time.delta_secs();
        }
    }
}

fn apply_axis_movement(
    axis_input: i8,
    velocity_axis: &mut f32,
    acceleration: f32,
    deceleration: f32,
    max_speed: f32,
    stop_threshold: f32,
) {
    if axis_input != 0 {
        *velocity_axis += acceleration * axis_input as f32;
        *velocity_axis = velocity_axis.clamp(-max_speed, max_speed);
    } else if velocity_axis.abs() > stop_threshold {
        let potential_new_velocity =
            *velocity_axis - deceleration * velocity_axis.signum();
        if potential_new_velocity.signum() == velocity_axis.signum() {
            *velocity_axis = potential_new_velocity;
        } else {
            *velocity_axis = 0.0;
        }
    } else {
        *velocity_axis = 0.0;
    }
}
