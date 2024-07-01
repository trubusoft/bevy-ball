use bevy::app::{App, Startup, Update};
use bevy::prelude::Plugin;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_player)
            .add_systems(Update, systems::player_movement)
            .add_systems(Update, systems::confine_player_movement)
            .add_systems(Update, systems::player_hit_enemy)
            .add_systems(Update, systems::player_hit_star);
    }
}
