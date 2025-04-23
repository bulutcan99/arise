use bevy::app::Plugin;

// Player logicleri bu kisimda yer alicak
pub mod asset;
pub mod movement;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
    }
}
