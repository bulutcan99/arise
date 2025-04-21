/// Tracks experience points and leveling threshold
#[derive(Component, Debug, Clone)]
pub struct Experience {
    pub current: u32,
    pub needed_for_next: u32,
}

/// Represents an entity's current level
#[derive(Component, Debug, Clone, Copy)]
pub struct Level(pub u32);
