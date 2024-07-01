use bevy::app::{App, Plugin, Startup, Update};

use crate::system::events::GameOver;

pub mod components;
pub mod events;
mod systems;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_systems(Startup, systems::spawn_camera)
            .add_systems(Update, systems::on_escape_exit);
    }
}
