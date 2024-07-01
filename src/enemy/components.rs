use bevy::math::Vec3;
use bevy::prelude::Component;

use crate::helpers::RandomHelper;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

impl Enemy {
    pub fn randomize_direction() -> Vec3 {
        Vec3::new(RandomHelper::random_f32(), RandomHelper::random_f32(), 0.0).normalize()
    }
}
