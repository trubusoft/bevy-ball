use bevy::prelude::{DetectChanges, EventReader, Res, ResMut};

use crate::score::components::{HighScore, Score};
use crate::system::events::GameOver;

pub fn print_score_on_change(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value);
    }
}

pub fn handle_game_over_event(mut event_reader: EventReader<GameOver>) {
    for event in event_reader.read() {
        println!("Your final score is: {}", event.score)
    }
}

pub fn update_high_score(
    mut event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for event in event_reader.read() {
        let last_score = event.score;
        high_score.scores.push(("Player".to_string(), last_score));
    }
}

pub fn print_high_score_on_change(high_score: Res<HighScore>) {
    if high_score.is_changed() {
        println!("High Score updated: {:?}", high_score);
    }
}
