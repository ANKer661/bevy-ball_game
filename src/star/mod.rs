use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub const NUMBER_OF_STAR: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_star)
            .add_systems(Update, tick_star_spawn)
            .add_systems(Update, spawn_star_overtime);
    }
}
