use arise_engine::states::AppStates;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
    commands.spawn((
        Sprite::from_image(player_asset.player_image.clone()),
        Transform::from_xyz(0., -150., 0.),
    ));
    // draw animated sprite using the texture atlas layout
    commands.spawn((
        Sprite::from_atlas_image(
            player_asset.player_image.clone(),
            TextureAtlas::from(player_asset.player_layout.clone()),
        ),
        Transform::from_xyz(0., 150., 0.),
        PlayerAnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )),
    ));
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAsset {
    #[asset(texture_atlas_layout(
        tile_size_x = 128 , // Düzeltildi: 1024 piksel genişlik / 8 sütun = 128
        tile_size_y = 128 , // Düzeltildi: 128 piksel yükseklik / 1 satır = 128
        columns = 8,       // Resme göre doğru görünüyor
        rows = 1,          // Resme göre doğru görünüyor
        padding_x = 0 ,    // Kareler arasında boşluk yok
        padding_y = 0 ,    // Kareler arasında boşluk yok (tek satır için önemsiz ama 0 iyi bir varsayılan)
        offset_x = 0 ,     // Resmin solundan başlama ofseti yok
        offset_y = 0       // Resmin üstünden başlama ofseti yok
    ))]
    pub player_layout: Handle<TextureAtlasLayout>,

    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "texture/player/Run.png")]
    pub player_image: Handle<Image>,
}

#[derive(Component)]
pub struct PlayerAnimationTimer(Timer);

pub fn animate_player_run_sprite(
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut Sprite)>,
) {
    for (mut animation_timer, mut sprite) in query.iter_mut() {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 8;
            }
        }
    }
}
