pub mod player;

use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::system::{Query, Res};
use bevy::prelude::{error, Resource};
use bevy::sprite::{Sprite, TextureAtlas, TextureAtlasLayout};
use bevy::state::condition::in_state;
use bevy::time::{Time, Timer, TimerMode};
use engine::animation::AnimationCompletedEvent;
use engine::states;
use serde::Deserialize;
use crate::animation::player::{player_handle_animation_change, PlayerAnimationChangeEvent};

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAnimationChangeEvent>();
        app.add_event::<AnimationCompletedEvent>();
        app.add_systems(
            Update,
            animate_sprite_system.run_if(in_state(states::AppStates::Game)),
        )
            .add_systems(Update, player_handle_animation_change.run_if(in_state(states::AppStates::Game)));
    }
}

/// A tag on entities that need to be animated
#[derive(Component)]
pub struct AnimationComponent {
    /// Timer to track frame duration,
    pub timer: Timer,
    /// Direction of the animation
    pub direction: AnimationDirection,
}

impl From<AnimationData> for AnimationComponent {
    fn from(data: AnimationData) -> Self {
        Self {
            timer: Timer::from_seconds(data.frame_duration, data.mode),
            direction: data.direction,
        }
    }
}

#[derive(Deserialize, Clone)]
pub enum AnimationDirection {
    None,
    Forward,
    PingPong(PingPongDirection),
}

#[derive(Deserialize, Clone)]
pub enum PingPongDirection {
    Forward,
    Backward,
}

/// Describes an animation
// TODO*: animation frame'i butun animationlar icin dynamic collection yapip animation'da calistirip burda res olarak cagir
#[derive(Resource, Deserialize)]
pub struct AnimationData {
    pub mode: TimerMode,
    pub direction: AnimationDirection,
    pub frame_duration: f32,
}

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut animation_complete_event_writer: EventWriter<AnimationCompletedEvent>,
    mut query: Query<(
        Entity,
        &mut AnimationComponent,
        &mut Sprite,
    )>,
) {
    for (entity, mut anim, mut sprite) in query.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            let just_finished = anim.timer.tick(time.delta()).just_finished();
            if !just_finished {
                continue;
            }

            if let Some(layout) = texture_atlas_layouts.get(atlas.layout.id()) {
                let num_frames = layout.len();

                match &mut anim.direction {
                    AnimationDirection::None => {}
                    AnimationDirection::Forward => {
                        let new_idx = (atlas.index + 1) % num_frames;
                        if new_idx == 0 {
                            animation_complete_event_writer
                                .send(AnimationCompletedEvent(entity));
                        }
                        atlas.index = new_idx;
                    }
                    AnimationDirection::PingPong(direction) => {
                        match direction {
                            PingPongDirection::Forward => {
                                if atlas.index >= num_frames - 1 {
                                    *direction = PingPongDirection::Backward;
                                    atlas.index = num_frames.saturating_sub(2);
                                } else {
                                    atlas.index += 1;
                                }
                            }
                            PingPongDirection::Backward => {
                                if atlas.index == 0 {
                                    *direction = PingPongDirection::Forward;
                                    atlas.index = 1;
                                } else {
                                    atlas.index -= 1;
                                }
                            }
                        }
                    }
                }
            } else {
                error!(
                    "Could not get texture atlas layout for id: {}",
                    atlas.layout.id()
                );
            }
        }
    }
}
