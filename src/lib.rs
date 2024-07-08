use bevy::app::{App, AppExit, Plugin, PostUpdate, Startup, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{
    Camera2dBundle, Commands, Component, default, Entity, EventReader, EventWriter, info, KeyCode,
    NextState, Query, Res, ResMut, State, States, Window, With,
};
use bevy::window::PrimaryWindow;

use game::player::CollidedWithEnemy;

use crate::helpers::WindowHelper;

pub mod game;
pub mod helpers;
pub mod ui;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, exit_on_escape)
            .add_systems(Update, transition_in_game_state)
            .add_systems(Update, transition_main_menu_state)
            .add_systems(Update, transition_game_over)
            .add_systems(PostUpdate, cleanup_entity);
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Component)]
pub struct ScheduleDespawn {}

impl Default for ScheduleDespawn {
    fn default() -> Self {
        Self {}
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.get_single() {
        commands.spawn(Camera2dBundle {
            transform: WindowHelper::center(window),
            ..default()
        });
    }
}

pub fn exit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writter: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writter.send(AppExit);
    }
}

pub fn cleanup_entity(mut commands: Commands, query: Query<Entity, With<ScheduleDespawn>>) {
    // Despawn entity that has been tagged so
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn transition_in_game_state(
    mut next_state: ResMut<NextState<ApplicationState>>,
    button_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<ApplicationState>>,
) {
    if button_input.just_pressed(KeyCode::KeyG) {
        match current_state.get() {
            ApplicationState::MainMenu | ApplicationState::GameOver => {
                next_state.set(ApplicationState::InGame);
                info!("{:?}", ApplicationState::InGame);
            }
            _ => {}
        }
    }
}

pub fn transition_main_menu_state(
    mut next_state: ResMut<NextState<ApplicationState>>,
    button_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<ApplicationState>>,
) {
    if button_input.just_pressed(KeyCode::KeyM) {
        match current_state.get() {
            ApplicationState::InGame | ApplicationState::GameOver => {
                next_state.set(ApplicationState::MainMenu);
                info!("{:?}", ApplicationState::MainMenu);
            }
            _ => {}
        }
    }
}

pub fn transition_game_over(
    mut next_state: ResMut<NextState<ApplicationState>>,
    mut event_reader: EventReader<CollidedWithEnemy>,
) {
    for _event in event_reader.read() {
        next_state.set(ApplicationState::GameOver);
        info!("{:?}", ApplicationState::GameOver);
    }
}
