use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use engine::animation::states::{AnimationChangeEvent, AnimationState};
use engine::input::PlayerAction;
use engine::player::{PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent};
use crate::game::resources::GameResource;

pub fn movement_system(
    time: Res<Time>, // Zamanı almak için Time resource'unu ekliyoruz
    game_parameters: Res<GameResource>,
    mut animation_events: EventWriter<AnimationChangeEvent>,
    mut player_query: Query<(
        Entity,
        &PlayerMobilityComponent,
        &mut Transform,
        &ActionState<PlayerAction>,
        &AnimationState,
        &mut PlayerVelocityComponent, // Yeni PlayerVelocity component'ını ekliyoruz
    ), With<PlayerComponent>>,
) {
    for (
        entity,
        player_mobility,
        mut transform,
        action_state,
        current_animation_state,
        mut player_velocity, // PlayerVelocity'yi mutable olarak alıyoruz
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
            _ => transform.scale.x, // Mevcut ölçeği koru eğer hareket yoksa veya sadece dikey hareket varsa
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
            // Artık kendi PlayerVelocity component'ımızı kullanıyoruz
            if player_velocity.0.abs() < game_parameters.stop_threshold &&
                player_velocity.1.abs() < game_parameters.stop_threshold {
                AnimationState::Idle
            } else {
                // Hareket girdisi yok ama hala kayıyor (decelerate oluyor)
                AnimationState::Running // Durana kadar koşma animasyonu devam eder
            }
        };

        // Eğer hedef animasyon durumu mevcut durumdan farklıysa event gönder
        if *current_animation_state != target_animation_state {
            animation_events.send(AnimationChangeEvent {
                entity,
                state: target_animation_state,
            });
        }

        // Hızı güncelle (PlayerVelocity component'ı üzerinde)
        apply_axis_movement(
            x_axis,
            &mut player_velocity.0, // PlayerVelocity.x'i güncelle
            player_mobility.acceleration.x,
            player_mobility.deceleration.x,
            player_mobility.speed.x,
            game_parameters.stop_threshold,
        );

        apply_axis_movement(
            y_axis,
            &mut player_velocity.1, // PlayerVelocity.y'yi güncelle
            player_mobility.acceleration.y,
            player_mobility.deceleration.y,
            player_mobility.speed.y,
            game_parameters.stop_threshold,
        );

        // Hızı Transform'a uygula (delta time ile çarparak)
        // Bu, hareketi kare hızından bağımsız hale getirir.
        transform.translation.x += player_velocity.0 * time.delta_secs();
        transform.translation.y += player_velocity.1 * time.delta_secs();

        // İsteğe bağlı: Zıplama vs. için yerçekimi gibi şeyler burada eklenebilir
        // player_velocity.y -= GRAVITY * time.delta_seconds();
        // Sonra transform.translation.y güncellenir.
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
        let potential_new_velocity = *velocity_axis - deceleration * velocity_axis.signum();
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