use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{
    in_state, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Plugin, Res, ResMut, State,
    States, Update,
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
        app.init_state::<GameState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(OnEnter(ApplicationState::InGame), resume_game)
            .add_systems(OnExit(ApplicationState::InGame), pause_game)
            .add_systems(
                Update,
                toggle_pause.run_if(in_state(ApplicationState::InGame)),
            );
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

pub fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Running => {
                pause_game(next_state);
            }
            GameState::Paused => {
                resume_game(next_state);
            }
        }
    }
}

pub fn pause_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Paused);
    info!("{:?}", GameState::Paused);
}

pub fn resume_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Running);
    info!("{:?}", GameState::Running);
}
