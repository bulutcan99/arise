use bevy::prelude::*;
use bevy::scene::ron::de::from_bytes;
use resources::GameResource;

pub mod counters;
pub mod resources;

pub struct GameResourcePlugin;

impl Plugin for GameResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<GameResource>(include_bytes!(
                "../../assets/data/game_parameters.ron"
            ))
            .unwrap(),
        );
    }
}
