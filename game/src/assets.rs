use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::appstate::AppState;

/// A plugin responsible for loading and managing game assets.
///
/// This plugin utilizes `bevy_asset_loader` to automatically load all required
/// images and audio assets at the start of the game. It ensures that assets are
/// available before entering gameplay by using a designated `AssetsLoading` state.
///
/// Specifically, it loads:
/// - `Images`: Textures such as player sprites, enemy sprites, and UI elements.
/// - `AudioAssets`: Sound effects and background music.
///
/// By organizing assets into collections and loading them during a dedicated state,
/// this plugin prevents runtime asset-loading issues and improves load-time
/// management.
///
/// # How it works
/// - When the game enters the `AssetsLoading` state, all asset collections are loaded.
/// - After all assets are fully loaded, the game can transition to the next state (e.g., `InGame`).
///
/// # Usage
/// Simply add the `AssetLoader` plugin to your app:
///
/// ```rust
/// app.add_plugins(AssetLoaderPlugin);
/// ```
///
/// Make sure that `AppState::AssetsLoading` exists and is properly handled in your state transitions.
///
/// # Requirements
/// - Assets must be placed at the correct relative paths.
/// - Ensure that the `AppState` enum includes a variant for asset loading (e.g., `AssetsLoading`).
///
/// # Benefits
/// - No manual loading code inside systems
/// - Cleaner startup phase
/// - Safer and faster asset management
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetsLoading)
                .load_collection::<ImageAssets>()
                .load_collection::<AudioAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "models/samurai.png")]
    pub samurai: Handle<Image>,
    #[asset(path = "models/blob.png")]
    pub blob: Handle<Image>,
    #[asset(path = "models/blob_death.png")]
    pub blob_death: Handle<Image>,
    #[asset(path = "models/slash_attack.png")]
    pub slash_attack: Handle<Image>,
    #[asset(path = "models/health_potion.png")]
    pub health_potion: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sounds/health_down.ogg")]
    pub health_down: Handle<AudioSource>,
    #[asset(path = "sounds/slash_attack.ogg")]
    pub slash_attack: Handle<AudioSource>,
    #[asset(path = "sounds/background_track.ogg")]
    pub background_track: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct MainMenuAssets {
    #[asset(key = "ui/background")]
    pub background: Handle<Image>,
}

#[derive(Component)]
pub struct PlayerHitSound {
    pub timer: Timer,
}
