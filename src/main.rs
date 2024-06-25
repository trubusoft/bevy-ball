use bevy::DefaultPlugins;
use bevy::prelude::{
    App, AssetServer, ButtonInput, Camera2dBundle, Commands, Component, default, KeyCode, Query,
    Res, SpriteBundle, Startup, Time, Transform, Update, Window, With,
};
use bevy::window::PrimaryWindow;

use bevy_ball::{MovementHelper, WindowHelper};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: WindowHelper::center(window),
        ..default()
    });
}

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: WindowHelper::center(window),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let movement_direction = MovementHelper::handle_input(keyboard_input);
        transform.translation += movement_direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let confined_translation =
            MovementHelper::confine(window, player_transform.translation, PLAYER_SIZE);
        player_transform.translation = confined_translation;
    }
}
