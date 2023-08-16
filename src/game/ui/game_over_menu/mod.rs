use crate::AppState;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layouts::*;
use systems::updates::*;

pub mod components;
pub mod styles;
pub mod systems;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // on enter state system
            .add_systems(OnEnter(AppState::GameOver), spawn_gameover_menu)
            // systems
            .add_systems(
                Update,
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    update_final_score
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            // on exit state system
            .add_systems(OnExit(AppState::GameOver), despawn_gameover_menu);
    }
}
