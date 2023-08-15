use bevy::prelude::*;

use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud_entity = build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // === Star Counter ===
            parent
                .spawn((
                    NodeBundle {
                        style: COUNTER_STYLE,
                        ..default()
                    },
                    StarCounter { star_number: 0 },
                ))
                .with_children(|parent| {
                    // star image
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });
                    // star number (score)
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "0".to_string(),
                                get_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Enemy Counter ===
            parent
                .spawn((
                    NodeBundle {
                        style: COUNTER_STYLE,
                        ..default()
                    },
                    EnemyCounter { enemy_number: 0 },
                ))
                .with_children(|parent| {
                    // enmey number
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "0".to_string(),
                                get_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // enemy image
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });
        })
        .id();
    hud_entity
}
