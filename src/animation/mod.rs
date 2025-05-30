use std::time::Duration;
use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::asset::ron::de::from_bytes;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::system::{Query, Res};
use bevy::prelude::{error, Resource};
use bevy::sprite::{Sprite, TextureAtlas, TextureAtlasLayout};
use bevy::state::condition::in_state;
use bevy::time::{Time, Timer, TimerMode};
use engine::states;
use serde::Deserialize;
use engine::animation::AnimationComponent;
use engine::animation::states::AnimationChangeEvent;
use crate::animation::animation::AnimationsResource;
use crate::player::character::CharactersResource;

pub mod player;
pub mod animation;
pub mod handler;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationChangeEvent>();

        app.insert_resource(
            from_bytes::<AnimationsResource>(include_bytes!("../../assets/data/animations.ron"))
                .unwrap(),
        );

    }
}

/*

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

 */