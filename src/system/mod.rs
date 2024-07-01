use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Camera2dBundle, Commands, default, Query, Window, With};
use bevy::window::PrimaryWindow;

use crate::helpers::WindowHelper;
use crate::system::events::GameOver;

pub mod events;
mod systems;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, systems::exit_on_escape);
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: WindowHelper::center(window),
        ..default()
    });
}
