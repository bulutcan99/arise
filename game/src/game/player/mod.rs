use bevy::app::Plugin;

// Player logicleri bu kisimda yer alicak
pub mod assets;
pub mod input;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
    }
}
