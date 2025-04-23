use bevy::{
    app::{App, Plugin},
    input::keyboard::KeyCode,
    reflect::Reflect,
};
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::{InputMap, VirtualAxis, VirtualDPad},
    Actionlike, InputManagerBundle,
};

pub struct PlayerActionPlugin;

impl Plugin for PlayerActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Move,
    Attack,
    Dash,
    Skill1,
    Skill2,
    Skill3,
}

impl Action {
    pub fn input_manager_bundle() -> InputManagerBundle<Self> {
        InputManagerBundle::<Self>::with_map(Self::input_map())
    }

    pub fn input_map() -> InputMap<Self> {
        let mut map = InputMap::default().with_dual_axis(
            Action::Move,
            VirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD),
        ); // WASD yon
        map.insert(Action::Attack, KeyCode::Space); // Space ile saldırı
        map.insert(Action::Dash, KeyCode::ShiftLeft); // Shift ile dash
        map.insert(Action::Skill1, KeyCode::KeyQ); // Q ile Skill1
        map.insert(Action::Skill2, KeyCode::KeyE); // E ile Skill2
        map.insert(Action::Skill3, KeyCode::KeyR); // R ile Skill3

        map
    }
}
