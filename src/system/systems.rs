use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Camera2dBundle, Commands, default, Entity, EventWriter, KeyCode, NextState, Query, Res, State,
    Window, With,
};
use bevy::window::PrimaryWindow;

use crate::ApplicationState;
use crate::helpers::WindowHelper;
use crate::system::components::Despawn;

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

pub fn transition_to_ingame_state(
    mut commands: Commands,
    button_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<ApplicationState>>,
) {
    if button_input.just_pressed(KeyCode::KeyG) {
        println!("Entered ApplicationState::InGame state");
        match state.get() {
            ApplicationState::MainMenu => {
                commands.insert_resource(NextState(Some(ApplicationState::InGame)));
            }
            ApplicationState::InGame => {}
            ApplicationState::GameOver => {
                commands.insert_resource(NextState(Some(ApplicationState::InGame)));
            }
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    button_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<ApplicationState>>,
) {
    if button_input.just_pressed(KeyCode::KeyM) {
        println!("Entered ApplicationState::MainMenu state");
        match state.get() {
            ApplicationState::MainMenu => {}
            ApplicationState::InGame => {
                commands.insert_resource(NextState(Some(ApplicationState::MainMenu)));
            }
            ApplicationState::GameOver => {
                commands.insert_resource(NextState(Some(ApplicationState::MainMenu)));
            }
        }
    }
}
