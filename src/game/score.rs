use bevy::app::{App, Update};
use bevy::prelude::{
    Commands, DetectChanges, EventReader, in_state, IntoSystemConfigs, NextState, OnEnter, OnExit,
    Plugin, Res, ResMut,
};
use bevy::prelude::Resource;

use crate::ApplicationState;
use crate::game::events::PlayerDead;
use crate::game::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::InGame), insert_score)
            .add_systems(OnExit(ApplicationState::InGame), remove_score)
            .add_systems(
                Update,
                on_score_change
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            .init_resource::<HighScore>()
            .add_systems(Update, on_player_dead)
            .add_systems(Update, update_high_score)
            .add_systems(Update, on_high_score_change);
    }
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScore {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

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
