use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // resource
            .init_resource::<HighScore>()
            // on enter state
            .add_systems(OnEnter(AppState::Game), insert_score)
            // systems
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_high_score)
            .add_systems(Update, high_score_updated)
            // on exit state
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
