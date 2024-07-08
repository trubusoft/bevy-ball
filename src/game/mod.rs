use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Commands, in_state, IntoSystemConfigs, KeyCode, NextState, Plugin, Res, State, States, Update,
};

use crate::ApplicationState;
use crate::game::enemy::EnemyPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::score::ScorePlugin;
use crate::game::star::StarPlugin;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(
                Update,
                toggle_simulation_state.run_if(in_state(ApplicationState::InGame)),
            );
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

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
