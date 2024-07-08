use bevy::prelude::{App, DetectChanges, EventReader, info, Plugin, Res, ResMut, Resource, Update};

use crate::game::player::CollidedWithEnemy;

pub struct HighScorePlugin;

impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_systems(Update, on_collided_with_enemy_update_high_score)
            .add_systems(Update, on_high_score_change_print);
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

pub fn on_collided_with_enemy_update_high_score(
    mut event_reader: EventReader<CollidedWithEnemy>,
    mut high_score: ResMut<HighScore>,
) {
    for event in event_reader.read() {
        let current_final_score = event.score;
        info!("Your final score is: {}", current_final_score);
        high_score
            .scores
            .push(("Player".to_string(), current_final_score));
    }
}

pub fn on_high_score_change_print(high_score: Res<HighScore>) {
    if high_score.is_changed() {
        info!("High Score updated: {:?}", high_score);
    }
}
