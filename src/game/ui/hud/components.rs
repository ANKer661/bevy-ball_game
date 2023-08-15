use bevy::prelude::Component;

#[derive(Component)]
pub struct HUD {}

#[derive(Component)]
pub struct StarCounter {
    pub star_number: u32,
}

#[derive(Component)]
pub struct EnemyCounter {
    pub enemy_number: u32,
}
