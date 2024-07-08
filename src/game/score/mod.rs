use bevy::app::{App, Update};
use bevy::prelude::{Commands, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin};

use crate::ApplicationState;
use crate::game::score::components::{HighScore, Score};
use crate::game::SimulationState;

pub mod components;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::InGame), insert_score)
            .add_systems(OnExit(ApplicationState::InGame), remove_score)
            .add_systems(
                Update,
                systems::on_score_change
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            .init_resource::<HighScore>()
            .add_systems(Update, systems::on_event_game_over)
            .add_systems(Update, systems::update_high_score)
            .add_systems(Update, systems::on_high_score_change);
    }
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}
