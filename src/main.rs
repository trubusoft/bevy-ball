use bevy::DefaultPlugins;
use bevy::prelude::{
    App, AssetServer, Camera2dBundle, Commands, Component, default, Query, Res, SpriteBundle,
    Startup, Transform, Window, With,
};
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .run();
}

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let middle_x = window.width() / 2.0;
    let middle_y = window.height() / 2.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(middle_x, middle_y, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let middle_x = window.width() / 2.0;
    let middle_y = window.height() / 2.0;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(middle_x, middle_y, 0.0),
        ..default()
    });
}
