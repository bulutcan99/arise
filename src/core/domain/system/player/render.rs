use crate::core::domain::component::render::AnimationConfig;
use crate::core::domain::entity::player::Player;
use bevy::input::ButtonInput;
use bevy::log;
use bevy::prelude::{KeyCode, Query, Res, Sprite, Time, With};

pub fn trigger_animation(
    mut query: Query<&mut AnimationConfig, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Some(key) = input.get_just_pressed() {
        match key {
            KeyCode::ArrowDown => {
                for mut animation_config in query.iter_mut() {
                    animation_config.frame_timer =
                        AnimationConfig::timer_from_fps(animation_config.fps);
                    log::info!("Arrow Down pressed!");
                }
            }
            KeyCode::ArrowLeft => {
                for mut animation_config in query.iter_mut() {
                    animation_config.frame_timer =
                        AnimationConfig::timer_from_fps(animation_config.fps);
                    log::info!("Arrow Left pressed!");
                }
            }
            KeyCode::ArrowRight => {
                for mut animation_config in query.iter_mut() {
                    animation_config.frame_timer =
                        AnimationConfig::timer_from_fps(animation_config.fps);
                    log::info!("Arrow Right pressed!");
                }
            }
            KeyCode::ArrowUp => {
                for mut animation_config in query.iter_mut() {
                    animation_config.frame_timer =
                        AnimationConfig::timer_from_fps(animation_config.fps);
                    log::info!("Arrow Up pressed!");
                }
            }
            _ => {}
        }
    }
}

pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}
