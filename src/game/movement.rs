use bevy::prelude::{
    App, in_state, IntoSystemConfigs, Plugin, Query, Transform, Update, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::ApplicationState;
use crate::game::{Confined, GameState, Size};
use crate::helpers::MovementHelper;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (confine_movement)
                .run_if(in_state(ApplicationState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

pub fn confine_movement(
    mut query: Query<(&mut Transform, &Size), With<Confined>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        for (mut transform, size) in query.iter_mut() {
            let confined_translation =
                MovementHelper::confine(window, transform.translation, size.value);
            transform.translation = confined_translation;
        }
    }
}
