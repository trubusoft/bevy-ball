use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Camera2dBundle, Commands, default, Query, Window, With};
use bevy::window::PrimaryWindow;

use crate::helpers::WindowHelper;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: WindowHelper::center(window),
        ..default()
    });
}
