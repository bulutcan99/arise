/// Core stats: health, speed, damage, range, and attack speed
#[derive(Component, Debug, Clone)]
pub struct Stats {
    pub max_health: f32,
    pub current_health: f32,
    pub speed: Vec3,
    pub range: Vec3,
    pub damage: f32,
    pub attack_speed: f32,
}
