use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// -----------------------------------------------------------------------------
// Shadow Player Animation Assets: Combined
// -----------------------------------------------------------------------------

/// Asset collection for all shadow player animations (run and walk).
#[derive(AssetCollection, Resource, Debug)]
pub struct PlayerShadowAssets {
    // --- Run Animation ---
    /// The texture atlas layout used for the run animation (8 frames, 128x128 each).
    #[asset(key = "shadow.run.layout")]
    pub run_layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's run animation.
    #[asset(key = "shadow.run.image")]
    pub run_image: Handle<Image>,
}

