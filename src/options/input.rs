use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use serde::Deserialize;
use engine::input::{InputsResource, MainMenuExplorer, MenuAction, PlayerAction};

/// Spawns entity to track navigation over menus
pub fn spawn_menu_explorer_system(mut commands: Commands, inputs_res: Res<InputsResource>) {
    commands
        .spawn(InputManagerBundle::<MenuAction> {
            action_state: ActionState::default(),
            input_map: inputs_res.menu.clone(),
        })
        .insert(MainMenuExplorer);
}

#[derive(Deserialize)]
pub struct InputBindings {
    pub menu_keyboard: Vec<(MenuAction, KeyCode)>,
    pub player_keyboard: Vec<(PlayerAction, KeyCode)>,
    pub player_mouse: Vec<(PlayerAction, MouseButton)>,
}

impl From<InputBindings> for InputsResource {
    fn from(bindings: InputBindings) -> Self {
        InputsResource {
            menu: InputMap::new(bindings.menu_keyboard)
                .to_owned(),
            player_keyboard: InputMap::new(bindings.player_keyboard)
                .insert_multiple(bindings.player_mouse)
                .to_owned(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn get_input_bindings() -> InputBindings {
    use ron::from_str;
    use std::{env::current_dir, fs::read_to_string};

    let config_path = current_dir().unwrap().join("config");

    from_str::<InputBindings>(&read_to_string(config_path.join("input.ron")).unwrap()).unwrap()
}

#[cfg(target_arch = "wasm32")]
pub(super) fn get_input_bindings() -> InputBindings {
    use ron::de::from_bytes;

    from_bytes::<InputBindings>(include_bytes!("../../assets/data/input.ron")).unwrap()
}
