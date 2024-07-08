use bevy::prelude::{Commands, DetectChanges, EventReader, NextState, Res, ResMut};

use crate::ApplicationState;
use crate::game::score::components::{HighScore, Score};
use crate::game::SimulationState;
use crate::system::events::PlayerDead;

pub fn on_score_change(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value);
    }
}

pub fn on_player_dead(mut commands: Commands, mut event_reader: EventReader<PlayerDead>) {
    for event in event_reader.read() {
        println!("Your final score is: {}", event.score);
        commands.insert_resource(NextState(Some(ApplicationState::GameOver)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
    }
}

pub fn update_high_score(
    mut event_reader: EventReader<PlayerDead>,
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
