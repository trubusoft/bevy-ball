use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::{
    App, AssetServer, ButtonInput, Camera2dBundle, Commands, Component, default, KeyCode, Query,
    Res, SpriteBundle, Startup, Time, Transform, Update, Window, With,
};
use bevy::window::PrimaryWindow;

use bevy_ball::{MovementHelper, RandomHelper, WindowHelper};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, enemy_movement)
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
        Player {},
        SpriteBundle {
            transform: WindowHelper::center(window),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
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

const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;

#[derive(Component)]
struct Enemy {
    pub direction: Vec3,
}

impl Enemy {
    fn randomize_direction() -> Vec3 {
        Vec3::new(RandomHelper::random_f32(), RandomHelper::random_f32(), 0.0).normalize()
    }
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = RandomHelper::random_f32() * window.width();
        let random_y = RandomHelper::random_f32() * window.height();

        commands.spawn((
            Enemy {
                direction: Enemy::randomize_direction(),
            },
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        let enemy_direction = enemy.direction;
        enemy_transform.translation += enemy_direction * ENEMY_SPEED * time.delta_seconds();
    }
}
