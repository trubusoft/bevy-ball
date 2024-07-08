use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin};

use crate::ApplicationState;
use crate::game::SimulationState;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::InGame), systems::spawn_player)
            .add_systems(OnExit(ApplicationState::InGame), systems::despawn_player)
            .add_systems(
                Update,
                (systems::player_movement, systems::confine_player_movement)
                    .chain()
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (
                    systems::on_hit_star_emit_collide_event,
                    systems::on_star_collide_despawn_star,
                )
                    .chain()
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (
                    systems::on_player_hit_enemy,
                    systems::on_star_collide_play_star_despawn_sound,
                    systems::on_star_collide_event_add_score,
                )
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
