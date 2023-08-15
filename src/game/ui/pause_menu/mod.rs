use bevy::prelude::*;

pub mod components;
pub mod styles;
pub mod systems;

use crate::game::ui::pause_menu::systems::interactions::*;
use crate::game::ui::pause_menu::systems::layouts::{despawn_pause_menu, spawn_pause_menu};
use crate::game::SimulationState;
use crate::AppState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            .add_systems(
                Update,
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::Paused))
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
