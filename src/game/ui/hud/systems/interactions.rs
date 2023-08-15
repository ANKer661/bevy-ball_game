use bevy::prelude::*;

use crate::game::enemy::components::Enemy;
use crate::game::score::resources::Score;
use crate::game::ui::hud::components::*;

pub fn update_score(mut star_counter_query: Query<&mut StarCounter>, score: Res<Score>) {
    let mut star_counter = star_counter_query.get_single_mut().unwrap();
    star_counter.star_number = score.value;
}

pub fn update_enemy(mut enemy_counter_query: Query<&mut EnemyCounter>, enemy_query: Query<&Enemy>) {
    let mut enemy_number = 0;
    for _ in enemy_query.iter() {
        enemy_number += 1;
    }
    let mut enemy_counter = enemy_counter_query.get_single_mut().unwrap();
    enemy_counter.enemy_number = enemy_number;
}
