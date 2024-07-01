use bevy::app::{App, Startup, Update};
use bevy::prelude::Plugin;

use crate::enemy::components::EnemySpawnTimer;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, systems::spawn_enemies)
            .add_systems(Update, systems::enemy_movement)
            .add_systems(Update, systems::confine_enemy_movement)
            .add_systems(Update, systems::update_enemy_direction)
            .add_systems(Update, systems::tick_spawn_enemy_timer)
            .add_systems(Update, systems::spawn_enemy_overtime);
    }
}
