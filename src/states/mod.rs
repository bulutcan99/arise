use assets::player::shadow::PlayerShadowAssets;
use assets::spawnable::projectile::ProjectileAssets;
use assets::weapon::WeaponAssets;
use bevy::prelude::{
    apply_deferred, App, AppExtStates, IntoSystemSetConfigs, NextState,
    OnEnter, OnExit, Plugin, Res, ResMut, State,
};
use bevy_asset_loader::prelude::*;
use engine::states::AppStates;

use crate::player::spawn::spawn_player_system;
use crate::weapon::spawn_weapon_system;

/// Includes systems that handle state transitions for `AppStates` and `GameStates`. Also includes
/// an asset loading state.
pub(super) struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppStates::LoadingAssets)
                .continue_to_state(AppStates::GameInit)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "shadow_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "weapon_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "projectile_assets.assets.ron",
                )
                .load_collection::<PlayerShadowAssets>()
                .load_collection::<WeaponAssets>()
                .load_collection::<ProjectileAssets>(),
        );

        app.add_systems(
            OnEnter(AppStates::GameInit),
            (
                spawn_player_system,
                spawn_weapon_system,
                transition_to_ingame,
            ),
        );
    }
}

fn transition_to_ingame(mut state: ResMut<NextState<AppStates>>) {
    state.set(AppStates::InGame);
}

