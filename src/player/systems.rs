use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::input::ButtonInput;
use bevy::prelude::{
    Commands, default, Entity, EventWriter, KeyCode, Query, Res, ResMut, SpriteBundle, Time,
    Transform, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::enemy::components::{Enemy, ENEMY_SIZE};
use crate::helpers::{MovementHelper, SoundHelper, WindowHelper};
use crate::player::components::{Player, PLAYER_SIZE, PLAYER_SPEED};
use crate::score::components::Score;
use crate::star::components::{Star, STAR_SIZE};
use crate::system::events::GameOver;

pub fn spawn_player(
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

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let movement_direction = MovementHelper::handle_input(keyboard_input);
        transform.translation += movement_direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
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

pub fn on_player_hit_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut game_over_event_writter: EventWriter<GameOver>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            let actual_distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if actual_distance <= (player_radius + enemy_radius) {
                commands.spawn(AudioBundle {
                    source: asset_server.load(SoundHelper::game_over_sound()),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(player_entity).despawn();
                game_over_event_writter.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn on_player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            let actual_distance = player_transform
                .translation
                .distance(star_transform.translation);

            if actual_distance <= (player_radius + star_radius) {
                commands.spawn(AudioBundle {
                    source: asset_server.load(SoundHelper::obtain_star_sound()),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}
