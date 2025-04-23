use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    Build,
    Finished,
}
