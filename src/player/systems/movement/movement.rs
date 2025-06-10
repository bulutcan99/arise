use bevy::log::debug;
use bevy::prelude::*;
use engine::animation::states::{AnimationChangeEvent, AnimationState};
use engine::input::PlayerAction;
use engine::player::{
    PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent,
};
use leafwing_input_manager::prelude::*;

use crate::game::resources::GameResource;

pub fn movement_system(
    time: Res<Time>,
    game_parameters: Res<GameResource>,
    mut animation_events: EventWriter<AnimationChangeEvent>,
    mut player_query: Query<
        (
            Entity,
            &PlayerMobilityComponent,
            &mut Transform,
            &ActionState<PlayerAction>,
            &AnimationState,
            &mut PlayerVelocityComponent,
        ),
        With<PlayerComponent>,
    >,
) {
    for (
        entity,
        player_mobility,
        mut transform,
        action_state,
        current_animation_state,
        mut player_velocity,
    ) in player_query.iter_mut()
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

        // Sprite yönünü ayarla
        let desired_scale_x = match x_axis {
            1 => game_parameters.sprite_scale.abs(),
            -1 => -game_parameters.sprite_scale.abs(),
            _ => transform.scale.x,
        };

        if transform.scale.x != desired_scale_x {
            transform.scale.x = desired_scale_x;
        }

        let has_movement_input = x_axis != 0 || y_axis != 0;

        if has_movement_input {
            if *current_animation_state != AnimationState::Run {
                animation_events.send(AnimationChangeEvent {
                    entity,
                    state: AnimationState::Run,
                });
            }
        } else {
            debug!(
                "Hareket yok, animasyon durumu değişmedi: {:?}",
                current_animation_state
            );
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

        // Hızı pozisyona uygula
        transform.translation.x += player_velocity.0 * time.delta_secs();
        transform.translation.y += player_velocity.1 * time.delta_secs();
    }
}

// apply_axis_movement fonksiyonu aynı kalabilir, çünkü hala tek bir hız eksenini yönetiyor.
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
        // Hız sıfıra doğru yavaşlıyorsa, yavaşlama miktarını mevcut hızın işaretiyle çarp
        // Bu, hızın sıfırı geçip ters yönde ivmelenmesini önler.
        let potential_new_velocity =
            *velocity_axis - deceleration * velocity_axis.signum();
        if potential_new_velocity.signum() == velocity_axis.signum() {
            *velocity_axis = potential_new_velocity;
        } else {
            // Eğer yavaşlama hızı sıfırın ötesine taşıyacaksa, hızı doğrudan sıfırla
            *velocity_axis = 0.0;
        }
    } else {
        *velocity_axis = 0.0;
    }
}
