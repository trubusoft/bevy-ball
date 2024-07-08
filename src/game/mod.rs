use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{
    Commands, in_state, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Plugin, Res,
    ResMut, State, States, Update,
};

use crate::game::enemy::EnemyPlugin;
use crate::game::events::{CollidedWithStar, PlayerDead};
use crate::game::player::PlayerPlugin;
use crate::game::score::ScorePlugin;
use crate::game::star::StarPlugin;
use crate::systems::ApplicationState;

pub mod enemy;
pub mod events;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<CollidedWithStar>()
            .add_event::<PlayerDead>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(OnEnter(ApplicationState::InGame), resume_simulation)
            .add_systems(OnExit(ApplicationState::InGame), pause_simulation)
            .add_systems(
                Update,
                toggle_simulation.run_if(in_state(ApplicationState::InGame)),
            );
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                info!("{:?}", SimulationState::Paused);
            }
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
                info!("{:?}", SimulationState::Running);
            }
        }
    }
}

pub fn pause_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running);
}
