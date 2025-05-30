use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use engine::animation::AnimationData;
use engine::animation::states::AnimationState;

#[derive(Resource, Default, Deserialize)]
pub struct AnimationsResource {
    pub animations: HashMap<AnimationState, AnimationData>
}