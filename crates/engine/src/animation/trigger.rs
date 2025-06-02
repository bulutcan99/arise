use bevy::prelude::TimerMode;
use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(
    Deserialize,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    Copy
)]
pub enum AnimationTimerMode {
    Once,
    #[default]
    Repeating,
}

impl From<AnimationTimerMode> for TimerMode {
    fn from(value: AnimationTimerMode) -> Self {
        match value {
            AnimationTimerMode::Once => Self::Once,
            AnimationTimerMode::Repeating => Self::Repeating,
        }
    }
}


#[derive(
    Deserialize,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    Copy
)]
pub enum AnimationDirection {
    #[default]
    Forward,
    PingPong(PingPongDirection),
}

#[derive(
    Deserialize,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    Copy
)]
pub enum PingPongDirection {
    #[default]
    Forward,
    Backward,
}

