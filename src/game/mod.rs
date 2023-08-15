use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod star;
mod systems;
mod ui;

use crate::events::*;
use crate::AppState;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // events
            .add_event::<GameOver>()
            // on enter state
            //.add_systems(OnEnter(AppState::Game), pause_simulation)
            // plugins
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_plugins(GameUIPlugin)
            // systems
            .add_systems(Update, toogle_similation.run_if(in_state(AppState::Game)));
            // on exit state
            //.add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
