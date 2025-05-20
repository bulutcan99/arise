use arise_engine::states::AppStates;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use player::asset::PlayerAssetLoaderPlugin;
use states::AssetLoaderPlugin;

mod animation;
mod asset;
mod camera;
mod consts;
mod development;
mod player;
mod states;

// TODO: ortak animation ve ortak assetler icin bir yapi! sonrasinda pink monster hareketlendirme!
fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)));
    let bevy_plugins = DefaultPlugins;
    let bevy_plugins = bevy_plugins.set(ImagePlugin::default_nearest());
    // Default plugins
    app.add_plugins(bevy_plugins);
    app.init_state::<AppStates>();
    // Our plugins
    app.add_plugins((
        // AssetLoaderPlugin,
        PlayerAssetLoaderPlugin,
    ));
    app.run();
}
