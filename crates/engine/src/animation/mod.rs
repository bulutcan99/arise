pub mod trigger;

use bevy::prelude::*;
use serde::Deserialize;

use crate::animation::trigger::{
    AnimationDirection, AnimationTimerMode,
};

/// A tag on entities that need to be animated
#[derive(Component)]
pub struct AnimationComponent {
    /// Timer to track frame duration,
    pub timer: Timer,
    /// Direction of the animation
    pub direction: AnimationDirection,
    pub mode: AnimationTimerMode,
}

impl From<&AnimationData> for AnimationComponent {
    fn from(data: &AnimationData) -> Self {
        Self {
            timer: Timer::from_seconds(data.frame_duration, data.mode.into()),
            direction: data.direction,
            mode: data.mode,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct AnimationData {
    pub frame_duration: f32,
    pub mode: AnimationTimerMode,
    pub direction: AnimationDirection,
}
