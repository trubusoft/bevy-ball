use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, default, Query, Res, ResMut, SpriteBundle, Time, Transform, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::game::star::components::{NUMBER_OF_STARS, Star, StarSpawnTimer};
use crate::helpers::RandomHelper;

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
