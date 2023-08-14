use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use super::SimulationState;
use crate::AppState;
use resources::*;
use systems::*;

pub const NUMBER_OF_STAR: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // on enter state
            .add_systems(OnEnter(AppState::Game), spawn_star)
            // systems
            .add_systems(
                Update,
                (tick_star_spawn, spawn_star_overtime)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // on exit state
            .add_systems(OnExit(AppState::Game), despawn_star);
    }
}
