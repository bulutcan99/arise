use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// -----------------------------------------------------------------------------
// Shadow Player Animation Assets: Combined
// -----------------------------------------------------------------------------

/// Asset collection for all shadow player animations (run and walk).
#[derive(AssetCollection, Resource)]
pub struct PlayerShadowAnimations {
    // --- Walk Animation ---
    /// The texture atlas layout used for the walk animation (8 frames, 128x128 each).
    #[asset(key = "shadow.walk.layout")]
    pub walk_layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's walk animation.
    #[asset(key = "shadow.walk.image")]
    pub walk_image: Handle<Image>,

    // --- Run Animation ---
    /// The texture atlas layout used for the run animation (8 frames, 128x128 each).
    #[asset(key = "shadow.run.layout")]
    pub run_layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's run animation.
    #[asset(key = "shadow.run.image")]
    pub run_image: Handle<Image>,
}

