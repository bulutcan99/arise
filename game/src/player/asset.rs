use arise_engine::states::AppStates;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::animation::{AnimationComponent, AnimationDirection};

pub struct PlayerAssetLoaderPlugin;

impl Plugin for PlayerAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppStates::LoadingAssets)
                .continue_to_state(AppStates::InitializeRun)
                .load_collection::<PlayerAsset>(),
        )
        .add_systems(
            OnEnter(AppStates::InitializeRun),
            draw_player_atlas,
        )
        .add_systems(
            Update,
            animate_player_run_sprite
                .run_if(in_state(AppStates::InitializeRun)),
        );
    }
}

pub fn draw_player_atlas(
    mut commands: Commands,
    player_asset: Res<PlayerAsset>,
) {
    // TODO: burdan kaldirilcak
    commands.spawn(Camera2d);

    // draw animated sprite using the texture atlas layout
    commands.spawn((
        Sprite::from_atlas_image(
            player_asset.player_image.clone(),
            TextureAtlas::from(player_asset.player_layout.clone()),
        ),
        Transform::from_xyz(0., 150., 0.),
        AnimationComponent{
            direction: AnimationDirection::Forward,
            timer: Timer::from_seconds(),
        },
    ));
}
