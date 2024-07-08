use bevy::app::{App, Startup, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin};

use crate::ApplicationState;
use crate::game::enemy::components::EnemySpawnTimer;
use crate::game::SimulationState;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, systems::spawn_initial_enemies)
            .add_systems(
                Update,
                (
                    systems::enemy_movement,
                    systems::confine_enemy_movement,
                    systems::update_enemy_direction_when_out_of_bound,
                    systems::tick_spawn_enemy_overtime,
                    systems::spawn_enemy_overtime,
                )
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
