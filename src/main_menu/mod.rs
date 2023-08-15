use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use crate::AppState;
use systems::interactions::*;
use systems::layouts::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // on enter state systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // systems
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            // on exit state systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
