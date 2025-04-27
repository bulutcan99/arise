use bevy::prelude::*;

mod appstate;
mod assets;
mod camera;
mod consts;
mod development;
mod game;
mod gamestate;
mod map;
mod player;
mod ui;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)));
    let bevy_plugins = DefaultPlugins;
    let bevy_plugins = bevy_plugins.set(ImagePlugin::default_nearest());
    // Default plugins
    app.add_plugins(bevy_plugins);
    // Our plugins
    app.add_plugins((
        camera::CameraPlugin,
        assets::AssetLoaderPlugin,
    ));
    app.run();
}
