use bevy::app::App;
use bevy::prelude::{
    in_state, info, AppExtStates, ButtonInput, Component, EventReader, IntoSystemConfigs, KeyCode,
    NextState, OnEnter, OnExit, Plugin, Res, ResMut, State, States, Update,
};

use crate::game::enemy::EnemyPlugin;
use crate::game::high_score::HighScorePlugin;
use crate::game::movement::MovementPlugin;
use crate::game::player::{CollidedWithEnemy, PlayerPlugin};
use crate::game::score::ScorePlugin;
use crate::game::star::StarPlugin;
use crate::ApplicationState;

pub mod enemy;
pub mod high_score;
mod movement;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(MovementPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(HighScorePlugin)
            .add_systems(OnEnter(ApplicationState::InGame), resume_game)
            .add_systems(OnExit(ApplicationState::InGame), pause_game)
            .add_systems(
                Update,
                toggle_pause.run_if(in_state(ApplicationState::InGame)),
            )
            .add_systems(Update, on_collided_with_enemy_set_game_over);
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Stop,
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
            _ => {}
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

pub fn on_collided_with_enemy_set_game_over(
    mut event_reader: EventReader<CollidedWithEnemy>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for _event in event_reader.read() {
        game_state.set(GameState::Stop);
    }
}

#[derive(Component)]
pub struct Size {
    value: f32,
}

#[derive(Component)]
pub struct Confined;
