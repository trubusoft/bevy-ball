use bevy::app::{App, Update};
use bevy::prelude::Plugin;

use crate::score::components::{HighScore, Score};

pub mod components;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(Update, systems::on_event_game_over)
            .add_systems(Update, systems::update_high_score)
            .add_systems(Update, systems::on_score_change)
            .add_systems(Update, systems::on_high_score_change);
    }
}
