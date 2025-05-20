use bevy::prelude::*;

#[derive(Event)]
pub struct CyclePhaseEvent;

#[derive(Event)]
pub struct RunEndEvent {
    pub outcome: RunOutcomeType,
}

pub enum RunOutcomeType {
    Victory,
    Defeat(RunDefeatType),
}

pub enum RunDefeatType {
    PlayersDied,
    TimeExpired,
}
