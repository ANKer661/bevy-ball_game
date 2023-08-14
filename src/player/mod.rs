use bevy::prelude::*;

pub mod components;
mod systems;

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
            // startup system
            .add_systems(Startup, spwan_player)
            // systems
            .add_systems(Update, player_movement.in_set(MovementSystemSet))
            .add_systems(Update, confine_player_movement.in_set(MovementSystemSet))
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, player_hit_star);
    }
}
