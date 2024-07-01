use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::prelude::{Commands, Query, Res, ResMut, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;

use crate::enemy::components::{
    Enemy, ENEMY_SIZE, ENEMY_SPEED, EnemySpawnTimer, NUMBER_OF_ENEMIES,
};
use crate::helpers::{MovementHelper, SoundHelper};

pub fn spawn_initial_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        for _ in 0..NUMBER_OF_ENEMIES {
            commands.spawn(Enemy::at_randomized_location(window, &asset_server));
        }
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        let enemy_direction = enemy.direction;
        enemy_transform.translation += enemy_direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction_when_out_of_bound(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let half_unit_size = ENEMY_SIZE / 2.0;
        let x_min = 0.0 + half_unit_size;
        let x_max = window.width() - half_unit_size;
        let y_min = 0.0 + half_unit_size;
        let y_max = window.height() - half_unit_size;

        let new_translation = enemy_transform.translation;
        let mut is_direction_changed: bool = false;

        if new_translation.x <= x_min || new_translation.x >= x_max {
            enemy.direction.x *= -1.0;
            is_direction_changed = true;
        }
        if new_translation.y <= y_min || new_translation.y >= y_max {
            enemy.direction.y *= -1.0;
            is_direction_changed = true;
        }

        if is_direction_changed {
            commands.spawn(AudioBundle {
                source: asset_server.load(SoundHelper::bounce_sound()),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let confined_translation =
            MovementHelper::confine(window, enemy_transform.translation, ENEMY_SIZE);
        enemy_transform.translation = confined_translation;
    }
}

pub fn tick_spawn_enemy_overtime(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_overtime(
    mut commands: Commands,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.just_finished() {
        if let Ok(window) = window_query.get_single() {
            commands.spawn(Enemy::at_randomized_location(window, &asset_server));
        }
    }
}
