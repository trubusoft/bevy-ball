use bevy::app::{App, Plugin, Startup, Update};
use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Camera2dBundle, Commands, Component, default, Entity, EventWriter, KeyCode, NextState,
    PostUpdate, Query, Res, ResMut, State, States, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::helpers::WindowHelper;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, on_escape_exit)
            .add_systems(PostUpdate, despawn_entity)
            .add_systems(Update, transition_to_in_game_state)
            .add_systems(Update, transition_to_main_menu_state);
    }
}

#[derive(Component)]
pub struct Despawn {}

impl Default for Despawn {
    fn default() -> Self {
        Self {}
    }
}

pub fn on_escape_exit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writter: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writter.send(AppExit);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: WindowHelper::center(window),
        ..default()
    });
}

pub fn despawn_entity(mut commands: Commands, query: Query<Entity, With<Despawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn transition_to_in_game_state(
    mut application_next_state: ResMut<NextState<ApplicationState>>,
    button_input: Res<ButtonInput<KeyCode>>,
    current_application_state: Res<State<ApplicationState>>,
) {
    if button_input.just_pressed(KeyCode::KeyG) {
        match current_application_state.get() {
            ApplicationState::MainMenu | ApplicationState::GameOver => {
                application_next_state.set(ApplicationState::InGame);
            }
            _ => {}
        }
    }
}

pub fn transition_to_main_menu_state(
    mut application_next_state: ResMut<NextState<ApplicationState>>,
    button_input: Res<ButtonInput<KeyCode>>,
    current_application_state: Res<State<ApplicationState>>,
) {
    if button_input.just_pressed(KeyCode::KeyM) {
        match current_application_state.get() {
            ApplicationState::InGame | ApplicationState::GameOver => {
                println!("Entered ApplicationState::MainMenu state");
                application_next_state.set(ApplicationState::MainMenu);
            }
            _ => {}
        }
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
