use bevy::prelude::*;

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
