use bevy::prelude::Resource;
use bevy::utils::HashMap;
use serde::Deserialize;
use engine::character::{Character, CharacterType};

#[derive(Resource, Deserialize)]
pub struct CharactersResource {
    pub characters: HashMap<CharacterType, Character>,
}