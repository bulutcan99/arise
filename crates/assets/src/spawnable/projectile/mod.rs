use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use engine::spawnable::Faction;
use engine::spawnable::projectile::ProjectileType;

/// Asset collection for all weapons.
#[derive(AssetCollection, Resource, Debug)]
pub struct ProjectileAssets {
    /// The image for the shadow player's run animation.
    #[asset(key = "ally_bullet.layout")]
    pub ally_bullet_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet.image")]
    pub ally_bullet_image: Handle<Image>,
}

impl ProjectileAssets {
    /// Use a ProjectileType enum to access a texture atlas layout
    pub fn get_texture_atlas_layout(
        &self,
        projectile_type: &ProjectileType,
    ) -> Handle<TextureAtlasLayout> {
        match projectile_type {
            ProjectileType::Bullet(faction) => match faction {
                Faction::Ally => self.ally_bullet_layout.clone(),
            },
        }
    }

    /// Use a ProjectileType enum to access an image handle
    pub fn get_image(&self, projectile_type: &ProjectileType) -> Handle<Image> {
        match projectile_type {
            ProjectileType::Bullet(faction) => match faction {
                Faction::Ally => self.ally_bullet_image.clone(),
            },
        }
    }
}
