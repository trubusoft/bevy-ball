use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin};

use crate::ApplicationState;
use crate::game::SimulationState;

pub mod components;
mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::StarSpawnTimer>()
            .add_systems(
                OnEnter(ApplicationState::InGame),
                systems::spawn_initial_stars,
            )
            .add_systems(OnExit(ApplicationState::InGame), systems::despawn_all_stars)
            .add_systems(
                Update,
                (
                    systems::tick_spawn_stars_overtime,
                    systems::spawn_stars_overtime,
                )
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
