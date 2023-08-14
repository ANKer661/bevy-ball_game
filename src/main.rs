use bevy::prelude::*;

pub mod events;
mod game;
mod main_menu;
pub mod systems;

use game::GamePlugin;
use main_menu::MenuPlugin;
use systems::*;

fn main() {
    App::new()
        // bevy plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // my plugins
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        // starup systems
        .add_systems(Startup, spawn_camera)
        // systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run()
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
