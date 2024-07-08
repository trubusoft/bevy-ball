use bevy::app::{App, Update};
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Component, default, Entity, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin,
    Query, Res, ResMut, Resource, SpriteBundle, Time, Timer, TimerMode, Transform, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::{ApplicationState, ScheduleDespawn};
use crate::game::GameState;
use crate::helpers::RandomHelper;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(ApplicationState::InGame), spawn_initial_stars)
            .add_systems(OnExit(ApplicationState::InGame), despawn_all_stars)
            .add_systems(
                Update,
                (tick_spawn_stars_overtime, spawn_stars_overtime)
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_initial_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.get_single() {
        for _ in 0..NUMBER_OF_STARS {
            let random_x = RandomHelper::random_f32() * window.width();
            let random_y = RandomHelper::random_f32() * window.height();

            commands.spawn((
                Star {},
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
            ));
        }
    }
}

pub fn tick_spawn_stars_overtime(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_overtime(
    mut commands: Commands,
    star_spawn_timer: Res<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.just_finished() {
        if let Ok(window) = window_query.get_single() {
            let random_x = RandomHelper::random_f32() * window.width();
            let random_y = RandomHelper::random_f32() * window.height();

            commands.spawn((
                Star {},
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
            ));
        }
    }
}

pub fn despawn_all_stars(mut commands: Commands, query: Query<Entity, With<Star>>) {
    for enemy_entity in query.iter() {
        commands
            .entity(enemy_entity)
            .insert(ScheduleDespawn::default());
    }
}
