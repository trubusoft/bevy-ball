use bevy::prelude::{DetectChanges, EventReader, Res, ResMut};

use crate::score::components::{HighScore, Score};
use crate::system::events::GameOver;

pub fn on_score_change(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value);
    }
}

pub fn on_event_game_over(mut event_reader: EventReader<GameOver>) {
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

pub fn on_high_score_change(high_score: Res<HighScore>) {
    if high_score.is_changed() {
        println!("High Score updated: {:?}", high_score);
    }
}
