use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

/// A plugin responsible for setting up and managing application states.
///
/// `AppStatePlugin` handles the lifecycle of different game states by:
/// - Initializing the `AppState` state machine.
/// - Automatically despawning all entities marked with `StateDespawnMarker` when leaving a state.
/// - Handling special state transitions like restarting the game.
///
/// # How it works
/// - `AppState` enum defines all possible application screens (e.g., Main Menu, In-Game, Loading Screen).
/// - When exiting any state, entities with the `StateDespawnMarker` component are recursively despawned.
/// - When entering the `Restart` state, the app automatically transitions back into the `InGame` state.
///
/// # Why use it?
/// - Keeps the world clean by removing leftover entities when changing states.
/// - Simplifies state transitions and restart logic.
/// - Ensures a predictable and controlled lifecycle for screens and their resources.
///
/// # Usage
/// Simply add the `AppStatePlugin` to your app:
///
/// ```rust
/// app.add_plugins(AppStatePlugin);
/// ```
///
/// Then, when spawning entities for a specific state (e.g., menus, levels), tag them with `StateDespawnMarker`:
///
/// ```rust
/// commands.spawn((
///     YourComponent,
///     StateDespawnMarker,
/// ));
/// ```
///
/// These entities will be automatically despawned when leaving their associated state.
///
/// # Notes
/// - Requires the `enum_iterator` crate for iterating over all `AppState` variants.
/// - Reflect derive is included for potential editor support (e.g., Bevy Reflect inspector).
///
/// # Related Components
/// - [`StateDespawnMarker`]: Marks entities that should be removed on state change.
pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        for state in enum_iterator::all::<AppState>() {
            app.add_systems(
                OnExit(state),
                despawn_all_recursive::<With<StateDespawnMarker>>,
            );
        }
        app.add_systems(OnEnter(AppState::Restart), restart);
    }
}

/// State type: Which "screen" is the app in?
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, States)]
#[derive(Reflect)]
#[derive(enum_iterator::Sequence)]
pub enum AppState {
    /// Initial loading screen at startup
    #[default]
    AssetsLoading,
    /// Main Menu
    MainMenu,
    /// Gameplay
    InGame,
    Restart,
}

pub fn restart(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

/// Marker for entities that should be despawned on `AppState` transition.
///
/// Use this on entities spawned when entering specific states, that need to
/// be cleaned up when exiting.
#[derive(Component)]
pub struct StateDespawnMarker;

pub fn despawn_all_recursive<F: QueryFilter>(
    world: &mut World,
    query: &mut QueryState<Entity, F>,
) {
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        if let Ok(entity) = world.get_entity_mut(entity) {
            entity.despawn_recursive();
        }
    }
}
