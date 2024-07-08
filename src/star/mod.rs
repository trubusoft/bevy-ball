use bevy::app::{App, Startup, Update};
use bevy::prelude::Plugin;

pub mod components;
mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::StarSpawnTimer>()
            .add_systems(Startup, systems::spawn_initial_stars)
            .add_systems(Update, systems::tick_spawn_stars_overtime)
            .add_systems(Update, systems::spawn_stars_overtime);
    }
}
