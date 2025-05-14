use arise_engine::states::AppStates;
use bevy::prelude::*;

mod camera;
mod consts;
mod development;
mod player;

fn main() {
    let mut app = App::new();
    app.init_state::<AppStates>();
    app.insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)));
    let bevy_plugins = DefaultPlugins;
    let bevy_plugins = bevy_plugins.set(ImagePlugin::default_nearest());
    // Default plugins
    app.add_plugins(bevy_plugins);
    // Our plugins
    app.run();
}
