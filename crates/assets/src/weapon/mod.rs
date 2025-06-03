use bevy::prelude::*;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy_asset_loader::prelude::*;

/// Asset collection for all weapons.
#[derive(AssetCollection, Resource, Debug)]
pub struct WeaponAssets {
    /// The image for the shadow player's run animation.
    #[asset(key = "weapon.staff.image")]
    pub staff_image: Handle<Image>,
}
