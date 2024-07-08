use bevy::DefaultPlugins;
use bevy::prelude::App;

use bevy_ball::enemy::EnemyPlugin;
use bevy_ball::player::PlayerPlugin;
use bevy_ball::score::ScorePlugin;
use bevy_ball::star::StarPlugin;
use bevy_ball::system::SystemPlugin;

fn main() {
    // Debug::main();
    // Debug::debug_system_plugin();
    // Debug::debug_player();
    Debug::debug_player_and_star();
}

struct Debug;

#[allow(dead_code)]
impl Debug {
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
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
}
