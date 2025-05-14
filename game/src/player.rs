use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use bevy::image::Image;
use bevy_asset_loader::asset_collection::AssetCollection;

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 96,
        tile_size_y = 99,
        columns = 8,
        rows = 1
    ))]
    #[asset(path = "texture/player/Walk.png")]
    sprite: Handle<Image>,
}
