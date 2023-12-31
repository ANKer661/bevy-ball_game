use super::resources::*;
use crate::events::*;
use bevy::prelude::*;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}

pub fn update_high_score(
    mut game_over_envent_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScore>,
) {
    for event in game_over_envent_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score))
    }
}

pub fn high_score_updated(high_scores: Res<HighScore>) {
    if high_scores.is_changed() {
        println!("High socre: {:?}", high_scores);
    }
}
