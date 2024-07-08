use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Time, Transform,
    Window, With, Without,
};
use bevy::window::PrimaryWindow;

use crate::enemy::components::{Enemy, ENEMY_SIZE};
use crate::helpers::{AudioHelper, MovementHelper};
use crate::player::components::{Player, PLAYER_SIZE, PLAYER_SPEED};
use crate::score::components::Score;
use crate::star::components::{Star, STAR_SIZE};
use crate::system::components::Despawn;
use crate::system::events::{CollidedWithStar, GameOver};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.get_single() {
        commands.spawn(Player::at_center_of_the_screen(window, &asset_server));
    }
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
    mut score: Option<Res<Score>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let is_collided = MovementHelper::is_collided(
                PLAYER_SIZE,
                player_transform.translation,
                ENEMY_SIZE,
                enemy_transform.translation,
            );

            if is_collided {
                commands.spawn(AudioHelper::play_game_over_sound(&asset_server));
                commands.entity(player_entity).despawn();
                if let Some(score) = &mut score {
                    game_over_event_writter.send(GameOver { score: score.value });
                }
            }
        }
    }
}

pub fn on_hit_star_emit_collide_event(
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), (With<Star>, Without<Despawn>)>,
    mut event_writer: EventWriter<CollidedWithStar>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let is_collided = MovementHelper::is_collided(
                PLAYER_SIZE,
                player_transform.translation,
                STAR_SIZE,
                star_transform.translation,
            );

            if is_collided {
                event_writer.send(CollidedWithStar { star_entity });
            }
        }
    }
}

pub fn on_star_collide_despawn_star(
    mut commands: Commands,
    mut event_reader: EventReader<CollidedWithStar>,
) {
    for event in event_reader.read() {
        let star_entity = event.star_entity;
        if let Some(mut entity_commands) = commands.get_entity(star_entity) {
            entity_commands.insert(Despawn {});
        }
    }
}

pub fn on_star_collide_play_star_despawn_sound(
    mut commands: Commands,
    mut event_reader: EventReader<CollidedWithStar>,
    asset_server: Res<AssetServer>,
) {
    for _event in event_reader.read() {
        commands.spawn(AudioHelper::play_obtain_star_sound(&asset_server));
    }
}

pub fn on_star_collide_event_add_score(
    mut score: Option<ResMut<Score>>,
    mut event_reader: EventReader<CollidedWithStar>,
) {
    let Some(score) = &mut score else {
        return;
    };

    for _event in event_reader.read() {
        score.value += 1;
    }
}
