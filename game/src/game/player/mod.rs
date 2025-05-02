use bevy::app::Plugin;
use bevy::ecs::bundle::Bundle;

// Player logicleri bu kisimda yer alicak
pub mod abilities;
pub mod action;
pub mod character;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
    }
}

/// Bundle of all player-specific components
#[derive(Bundle)]
pub struct PlayerBundle {}
