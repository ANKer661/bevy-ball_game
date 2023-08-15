use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use pause_menu::PauseMenuPlugin;

mod game_over_menu;
mod hud;
mod pause_menu;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins
            .add_plugins(GameOverMenuPlugin)
            .add_plugins(PauseMenuPlugin);
    }
}
