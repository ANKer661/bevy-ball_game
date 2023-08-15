use bevy::prelude::*;

use crate::game::score::resources::HighScore;
use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::game_over_menu::styles::*;

pub fn spawn_gameover_menu(
    mut conmmands: Commands,
    asset_server: Res<AssetServer>,
    high_score: Res<HighScore>,
) {
    let _game_over_menu_entity = build_gameover_menu(&mut conmmands, &asset_server, &high_score);
}

pub fn despawn_gameover_menu(
    mut commands: Commands,
    gameover_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(gameover_menu_entity) = gameover_menu_query.get_single() {
        commands.entity(gameover_menu_entity).despawn_recursive();
    }
}

pub fn build_gameover_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    high_score: &Res<HighScore>,
) -> Entity {
    let game_over_entity = commands
        .spawn((
            NodeBundle {
                style: GAMEOVER_MENU_STYLE,
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over".to_string(),
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Score ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!(
                                    "Your final score was:\n {}",
                                    high_score.scores.last().unwrap().1 // score
                                )
                                .to_string(),
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Restart Button ==
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    RestartButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Restart".to_string(),
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Main Menu Button ==
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    MainMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Main Menu".to_string(),
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Quit Button ==
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit".to_string(),
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    game_over_entity
}
