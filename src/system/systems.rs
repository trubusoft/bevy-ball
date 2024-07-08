use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Camera2dBundle, Commands, default, Entity, EventWriter, KeyCode, Query, Res, Window, With,
};
use bevy::window::PrimaryWindow;

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
