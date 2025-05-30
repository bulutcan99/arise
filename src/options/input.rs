use std::fs::read_to_string;
use std::path::PathBuf;

use bevy::prelude::*;
use engine::input::{InputsResource, PlayerAction};
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::InputManagerBundle;
use ron::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputBindings {
    pub player_keyboard: Vec<(PlayerAction, KeyCode)>,
    pub player_mouse: Vec<(PlayerAction, MouseButton)>,
}

impl From<InputBindings> for InputsResource {
    fn from(bindings: InputBindings) -> Self {
        InputsResource {
            player_keyboard: InputMap::new(bindings.player_keyboard)
                .insert_multiple(bindings.player_mouse)
                .to_owned(),
        }
    }
}

pub fn get_input_bindings() -> InputBindings {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("assets");
    config_path.push("data");

    from_str::<InputBindings>(
        &read_to_string(config_path.join("input.ron")).unwrap(),
    )
    .unwrap()
}
