use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, States, Update};

use crate::game::enemy::EnemyPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::score::ScorePlugin;
use crate::game::star::StarPlugin;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin);
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
