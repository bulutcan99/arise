use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
/// Asset collection for all weapons.
#[derive(AssetCollection, Resource, Debug)]
pub struct ProjectileAssets {
    /// The image for the shadow player's run animation.
    #[asset(key = "projectile.bullet.image")]
    pub bullet: Handle<Image>,
}
