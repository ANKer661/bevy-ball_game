use bevy::prelude::*;

pub mod components;
pub mod styles;
pub mod systems;

use self::systems::layouts::*;
use self::systems::updates::*;
use crate::AppState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(
                Update,
                (update_score, update_enemy).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
