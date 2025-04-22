use bevy::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, BackgroundMovementScale, SetImageRepeatingExt,
};

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let image = asset_server.load("texture/background.png");
    commands.set_image_repeating(image.clone());

    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn backgrounds
    commands.spawn(BackgroundImageBundle::from_image(image, materials.as_mut()).at_z_layer(0.1));
}
