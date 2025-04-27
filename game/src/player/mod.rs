use bevy::prelude::*;

// Player logicleri bu kisimda yer alicak

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
    }
}

#[derive(Component)]
pub struct Player;
