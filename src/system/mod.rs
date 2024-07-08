use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::PostUpdate;

use crate::system::events::{CollidedWithStar, GameOver};

pub mod components;
pub mod events;
mod systems;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedWithStar>()
            .add_event::<GameOver>()
            .add_systems(Startup, systems::spawn_camera)
            .add_systems(Update, systems::on_escape_exit)
            .add_systems(PostUpdate, systems::despawn_entity);
    }
}
