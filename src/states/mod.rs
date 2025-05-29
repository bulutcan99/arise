use assets::player::shadow::PlayerShadowAssets;
use bevy::prelude::{App, IntoSystemSetConfigs, OnEnter, Plugin, Res, State};
use bevy_asset_loader::prelude::*;
use engine::states::{AppStates};

/// Includes systems that handle state transitions for `AppStates` and `GameStates`. Also includes
/// an asset loading state.
pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
	fn build(&self, app: &mut App) {
		app.add_loading_state(
			LoadingState::new(AppStates::LoadingAssets)
				.continue_to_state(AppStates::Game)
				.with_dynamic_assets_file::<StandardDynamicAssetCollection>(
					"shadow_assets.assets.ron",
				)
				.load_collection::<PlayerShadowAssets>(),
		);
		log::info!("Assets loaded");

		/*

		/// Configure the system sets that must run in order when entering the Game state.
		///
		/// This ensures the correct initialization sequence for the game world:
		/// - `Initialize`: Perform any core setup required before building the level.
		/// - `BuildLevel`: Load the map and level data. (The player cannot walk without a map!)
		/// - `SpawnPlayer`: Spawn the player into the world after the map exists.
		/// - `BuildUi`: Initialize and display the game UI after the world is ready.
		///
		/// These sets are chained to guarantee that each step is completed before the next begins.
		app.edit_schedule(OnEnter(AppStates::Game), |schedule| {
			schedule.configure_sets(
				(
					GameEnterSet::Initialize,
					GameEnterSet::BuildLevel,
					GameEnterSet::SpawnPlayer,
					GameEnterSet::BuildUi,
				)
					.chain(),
			);
		});

		 */
	}
}

