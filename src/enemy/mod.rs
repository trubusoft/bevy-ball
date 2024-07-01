use bevy::app::{App, Startup, Update};
use bevy::prelude::Plugin;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_enemies)
            .add_systems(Update, systems::enemy_movement)
            .add_systems(Update, systems::confine_enemy_movement)
            .add_systems(Update, systems::update_enemy_direction);
    }
}
