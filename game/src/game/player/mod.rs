use bevy::prelude::*;

pub mod abilities;
pub mod action;
pub mod character;
pub mod shadow_monarch;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
    }
}

/// Bundle of all player-specific components
#[derive(Bundle)]
pub struct PlayerBundle {}
