use bevy::prelude::{ButtonInput, Commands, KeyCode, NextState, Res, State};

use crate::game::SimulationState;

pub fn toggle_simulation_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("simulation paused");
            }
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
                println!("simulation running");
            }
        }
    }
}
