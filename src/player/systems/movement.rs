use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use engine::input::PlayerAction;
use engine::player::{PlayerComponent, PlayerMobilityComponent};
use leafwing_input_manager::prelude::ActionState;
use engine::animation::states::{AnimationChangeEvent, AnimationState};
use crate::game::resources::GameResource;


pub fn movement_system(
    game_parameters: Res<GameResource>,
    mut animation_events: EventWriter<AnimationChangeEvent>,
    // AnimationState component'ını sorguya ekliyoruz
    mut player_query: Query<(
        Entity, // Event için Entity ID'sine ihtiyacımız var
        &PlayerMobilityComponent,
        &mut Velocity, // bevy_rapier2d kullanıyorsanız &mut bevy_rapier2d::prelude::Velocity
        &mut Transform,
        &ActionState<PlayerAction>,
        &AnimationState, // Oyuncunun mevcut animasyon durumunu okumak için
    ), With<PlayerComponent>>, // Sadece PlayerComponent'e sahip entity'ler için çalışsın
) {
    for (
        entity,
        player_mobility,
        mut vel, // Eğer bevy_rapier2d::prelude::Velocity ise vel.linvel.x/y
        mut transform,
        action_state,
        current_animation_state, // Mevcut animasyon durumu
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

        // Hareket girdisi var mı?
        let has_movement_input = x_axis != 0 || y_axis != 0;

        // Hedef animasyon durumunu belirle
        let target_animation_state = if has_movement_input {
            AnimationState::Running
        } else {
            // Eğer hareket girdisi yoksa ve oyuncu neredeyse durmuşsa Idle
            // bevy_rapier2d::prelude::Velocity kullanıyorsanız vel.linvel.x ve vel.linvel.y olacak
            if vel.linvel.x.abs() < game_parameters.stop_threshold &&
                vel.linvel.y.abs() < game_parameters.stop_threshold {
                AnimationState::Idle
            } else {
                // Hareket girdisi yok ama hala kayıyor (decelerate oluyor)
                // Bu durumda hala Running animasyonunda kalabilir veya farklı bir "kayma" animasyonu olabilir.
                // Şimdilik Running'de kalmasını sağlayalım, böylece durana kadar koşma animasyonu devam eder.
                AnimationState::Running
            }
        };

        // Eğer hedef animasyon durumu mevcut durumdan farklıysa event gönder
        if *current_animation_state != target_animation_state {
            animation_events.send(AnimationChangeEvent {
                entity,
                state: target_animation_state,
            });
            // Not: Event gönderildikten sonra `current_animation_state` hemen güncellenmez.
            // Bu güncelleme `player_handle_animation_change` sisteminde olur.
        }

        // Hareketi uygula (Velocity component'ını Rapier kullanıyorsanız vel.linvel.x vb. olarak değiştirin)
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

// apply_axis_movement fonksiyonunuz aynı kalabilir
fn apply_axis_movement(
    axis_input: i8,
    velocity_axis: &mut f32, // velocity: &mut f32 idi, daha açıklayıcı olması için velocity_axis yaptım
    acceleration: f32,
    deceleration: f32,
    max_speed: f32,
    stop_threshold: f32,
) {
    if axis_input != 0 {
        *velocity_axis += acceleration * axis_input as f32;
        // Hızı max_speed ile sınırla
        *velocity_axis = velocity_axis.clamp(-max_speed, max_speed);
        // clamp kullanmak daha kısa: if velocity_axis.abs() > max_speed { *velocity_axis = velocity_axis.signum() * max_speed; }
    } else if velocity_axis.abs() > stop_threshold {
        // Girdi yoksa yavaşla
        *velocity_axis -= deceleration * velocity_axis.signum();
    } else {
        // Yeterince yavaşsa dur
        *velocity_axis = 0.0;
    }
}