use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// -----------------------------------------------------------------------------
// Shadow Player Animation Assets: Combined
// -----------------------------------------------------------------------------

/// Asset collection for all shadow player animations (run and walk).
#[derive(AssetCollection, Resource, Debug)]
pub struct PlayerShadowAssets {
    // --- Idle Animation ---
    /// The texture atlas layout used for the run animation (8 frames, 128x128 each).
    #[asset(key = "shadow.idle.layout")]
    pub idle_layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's run animation.
    #[asset(key = "shadow.idle.image")]
    pub idle_image: Handle<Image>,

    // --- Running Animation ---
    /// The texture atlas layout used for the run animation (8 frames, 128x128 each).
    #[asset(key = "shadow.run.layout")]
    pub run_layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's run animation.
    #[asset(key = "shadow.run.image")]
    pub run_image: Handle<Image>,

    // --- Light Attack Animation ---
    /// The texture atlas layout used for the run animation (8 frames, 128x128 each).
    #[asset(key = "shadow.light.attack.layout")]
    pub light_attack_layout: Handle<TextureAtlasLayout>,

    /// The image for the shadow player's run animation.
    #[asset(key = "shadow.light.attack.image")]
    pub light_attack_image: Handle<Image>,
}
