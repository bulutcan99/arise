use std::{thread::sleep, time::Duration};

use bevy::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, BackgroundMovementScale, SetImageRepeatingExt,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        //TODO: Map render cok stabil calismiyor burayi farkli bir sekidle yapmak lazim
        //TODO: Startup yerine stateler eklencek
        //TODO: Player Render ve animation yapilacak
        app.add_systems(Startup, setup);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    sleep(Duration::from_secs(1));
    let image = asset_server.load("texture/background.png");
    commands.set_image_repeating(image.clone());
    info!("BURAYA GIRDI");

    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn backgrounds
    commands.spawn(BackgroundImageBundle::from_image(image, materials.as_mut()).at_z_layer(0.1));
}
