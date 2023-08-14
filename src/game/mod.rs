use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod star;
mod systems;

use crate::events::*;
use crate::AppState;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // events
            .add_event::<GameOver>()
            // plugins
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            // systems
            .add_systems(Update, toogle_similation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
