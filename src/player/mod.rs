use bevy::app::{App, Startup, Update};
use bevy::prelude::{IntoSystemConfigs, Plugin};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_player)
            .add_systems(Update, systems::player_movement)
            .add_systems(Update, systems::confine_player_movement)
            .add_systems(Update, systems::on_player_hit_enemy)
            .add_systems(
                Update,
                (
                    systems::on_hit_star_emit_collide_event,
                    systems::on_star_collide_despawn_star,
                )
                    .chain(),
            )
            .add_systems(Update, systems::on_star_collide_play_star_despawn_sound)
            .add_systems(Update, systems::on_star_collide_event_add_score);
    }
}
