use bevy::prelude::*;

pub mod components;
mod systems;

use super::SimulationState;
use crate::AppState;
use systems::*;

pub const PLAYER_SIZE: f32 = 64.0; // This is player sprite size.
pub const PLAYER_SPEED: f32 = 500.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, MovementSystemSet.before(ConfinementSystemSet))
            // on enter system
            .add_systems(OnEnter(AppState::Game), spwan_player)
            // systems
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (enemy_hit_player, player_hit_star)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // on exit state
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
