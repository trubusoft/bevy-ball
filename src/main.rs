use bevy::DefaultPlugins;
use bevy::prelude::App;

use bevy_ball::{ApplicationState, SystemPlugin};
use bevy_ball::game::enemy::EnemyPlugin;
use bevy_ball::game::GamePlugin;
use bevy_ball::game::player::PlayerPlugin;
use bevy_ball::game::score::ScorePlugin;
use bevy_ball::game::star::StarPlugin;
use bevy_ball::ui::MainMenuPlugin;

fn main() {
    Debug::main();
    // Debug::debug_system_plugin();
    // Debug::debug_player();
    // Debug::debug_player_and_star();
    // Debug::debug_player_star_score();
    // Debug::debug_enemy();
}

struct Debug;

#[allow(dead_code)]
impl Debug {
    fn main() {
        App::new()
            .init_state::<ApplicationState>()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GamePlugin)
            .run();
    }

    fn debug_system_plugin() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .run();
    }

    fn debug_player() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .add_plugins(PlayerPlugin)
            .run();
    }

    fn debug_player_and_star() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .run();
    }

    fn debug_player_star_score() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .run();
    }

    fn debug_enemy() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .add_plugins(EnemyPlugin)
            .run();
    }
}
