use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// -----------------------------------------------------------------------------
// Shadow Player Animation Assets: Run
// -----------------------------------------------------------------------------

/// Asset collection for the shadow player's run animation.
#[derive(AssetCollection, Resource)]
pub struct PlayerShadowRunAsset {
    /// The texture atlas layout used for the run animation (8 frames, 128x128 each).
    #[asset(texture_atlas_layout(
        tile_size_x = 128,
        tile_size_y = 128,
        columns = 8,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's run animation.
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "texture/player/shadow/Run.png")]
    pub image: Handle<Image>,
}

// -----------------------------------------------------------------------------
// Shadow Player Animation Assets: Walk
// -----------------------------------------------------------------------------

/// Asset collection for the shadow player's walk animation.
#[derive(AssetCollection, Resource)]
pub struct PlayerShadowWalkAsset {
    /// The texture atlas layout used for the walk animation (8 frames, 128x128 each).
    #[asset(texture_atlas_layout(
        tile_size_x = 128,
        tile_size_y = 128,
        columns = 8,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's walk animation.
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "texture/player/shadow/Walk.png")]
    pub image: Handle<Image>,
}

// -----------------------------------------------------------------------------
// Combined Shadow Animation Assets
// -----------------------------------------------------------------------------

/// Container for all animations related to the shadow player.
#[derive(Resource)]
pub struct PlayerShadowAssets {
    /// Shadow run animation assets.
    pub run: PlayerShadowRunAsset,

    /// Shadow walk animation assets.
    pub walk: PlayerShadowWalkAsset,
}
