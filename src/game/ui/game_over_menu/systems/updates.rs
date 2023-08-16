use bevy::prelude::*;

use crate::game::score::resources::HighScore;
use crate::game::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score(
    high_score: Res<HighScore>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    let cur_score = high_score.scores.last().unwrap().1;
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Final Score: {}", cur_score.to_string());
    }
}
