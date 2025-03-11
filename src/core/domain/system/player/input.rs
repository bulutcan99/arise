use crate::core::domain::component::movement::Position;
use crate::core::domain::entity::player::Player;
use bevy::prelude::{ButtonInput, Commands, Entity, KeyCode, Query, ResMut, With};

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut game_log: ResMut<GameLog>,
    player_position: Query<(Entity, &Position), With<Player>>,
) {
}
