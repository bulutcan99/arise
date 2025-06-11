use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use engine::animation::AnimationData;
use engine::states::player::PlayerState;

#[derive(Resource, Default, Deserialize)]
pub struct AnimationsResource {
    pub animations: HashMap<PlayerState, AnimationData>
}