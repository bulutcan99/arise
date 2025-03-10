use crate::core::domain::component::render::AnimationConfig;
use crate::core::domain::component::warrior::Warrior;
use crate::core::domain::entity::player::Player;
use bevy::asset::{AssetServer, Assets};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{
    Camera2d, Commands, Res, ResMut, Sprite, TextureAtlas, TextureAtlasLayout, Transform,
};

pub fn spawn_user(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("user.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // the sprite runs at 10 FPS
    let animation_config = AnimationConfig::new(1, 6, 20);
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(50.0, 0.0, 0.0)),
        Player(1),
        Warrior::new(),
        animation_config,
    ));
}
