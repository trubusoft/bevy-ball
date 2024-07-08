use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{OnEnter, OnExit, PostUpdate};

use crate::ApplicationState;
use crate::system::events::{CollidedWithStar, PlayerDead};

pub mod components;
pub mod events;
mod systems;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedWithStar>()
            .add_event::<PlayerDead>()
            .add_systems(
                OnEnter(ApplicationState::InGame),
                systems::resume_simulation,
            )
            .add_systems(OnExit(ApplicationState::InGame), systems::pause_simulation)
            .add_systems(Startup, systems::spawn_camera)
            .add_systems(Update, systems::on_escape_exit)
            .add_systems(PostUpdate, systems::despawn_entity)
            .add_systems(Update, systems::transition_to_in_game_state)
            .add_systems(Update, systems::transition_to_main_menu_state);
    }
}
