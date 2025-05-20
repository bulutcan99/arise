/// Group of assets used for the player's run animation.
#[derive(AssetCollection, Resource)]
pub struct PlayerRunAsset {
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

    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "texture/player/Run.png")]
    pub image: Handle<Image>,
}

/// Top-level asset collection for all player-related animations.
#[derive(Resource)]
pub struct PlayerAsset {
    pub run: PlayerRunAsset,
}