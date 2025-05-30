use bevy::prelude::*;
use engine::input::InputsResource;
use input::get_input_bindings;

pub mod display;
pub mod input;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputsResource::from(
            get_input_bindings(),
        ));
    }
}
