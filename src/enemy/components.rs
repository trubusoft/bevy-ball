use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    Component, default, Res, Resource, SpriteBundle, Timer, TimerMode, Transform, Window,
};

use crate::helpers::{RandomHelper, SpriteHelper};

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
