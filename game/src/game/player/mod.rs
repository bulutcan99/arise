use bevy::prelude::*;

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

/// Identity of a player component, used for syncing UI
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum PlayerIDComponent {
    One,
    Two,
}

/// Useful for mapping an index to a PlayerIDComponent
impl From<usize> for PlayerIDComponent {
    fn from(value: usize) -> Self {
        match value {
            0 => PlayerIDComponent::One,
            _ => PlayerIDComponent::Two,
        }
    }
}
