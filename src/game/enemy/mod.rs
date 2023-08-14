use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use crate::game::SimulationState;
use crate::AppState;
use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMY: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is enemy sprite size.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // resource
            .init_resource::<EnemySpawnTimer>()
            // on enter state
            .add_systems(OnEnter(AppState::Game), spawn_enemy)
            // systems
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn,
                    spawn_enmey_overtime,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(AppState::Game)),
            )
            // on exit state
            .add_systems(OnExit(AppState::Game), despawun_enemy);
    }
}
