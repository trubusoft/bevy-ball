use bevy::prelude::{
    App, Commands, DetectChanges, EventReader, in_state, info, IntoSystemConfigs, NextState,
    OnEnter, OnExit, Plugin, Res, ResMut, Resource, Update,
};

use crate::ApplicationState;
use crate::game::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::InGame), insert_score_resource)
            .add_systems(OnExit(ApplicationState::InGame), remove_score_resource)
            .add_systems(
                Update,
                on_score_change_print
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
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

pub fn insert_score_resource(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score_resource(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn on_score_change_print(score: Res<Score>) {
    if score.is_changed() {
        info!("Score updated: {}", score.value);
    }
}
