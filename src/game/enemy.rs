use bevy::app::{App, Update};
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    Component, default, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Resource,
    SpriteBundle, Timer, TimerMode,
};
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;

use crate::{ApplicationState, Despawn};
use crate::game::SimulationState;
use crate::helpers::{AudioHelper, MovementHelper};
use crate::helpers::{RandomHelper, SpriteHelper};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(ApplicationState::InGame), spawn_initial_enemies)
            .add_systems(OnExit(ApplicationState::InGame), despawn_all_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement,
                    update_enemy_direction_when_out_of_bound,
                    tick_spawn_enemy_overtime,
                    spawn_enemy_overtime,
                )
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

impl Enemy {
    pub fn randomize_direction() -> Vec3 {
        Vec3::new(RandomHelper::random_f32(), RandomHelper::random_f32(), 0.0).normalize()
    }

    pub fn at_randomized_location(
        window: &Window,
        asset_server: &Res<AssetServer>,
    ) -> (Enemy, SpriteBundle) {
        let random_x = RandomHelper::random_f32() * window.width();
        let random_y = RandomHelper::random_f32() * window.height();

        (
            Enemy {
                direction: Self::randomize_direction(),
            },
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(SpriteHelper::enemy_sprite()),
                ..default()
            },
        )
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

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
            commands.spawn(AudioHelper::play_bounce_sound(&asset_server));
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

pub fn despawn_all_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for enemy_entity in query.iter() {
        commands.entity(enemy_entity).insert(Despawn {});
    }
}
