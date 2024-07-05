use bevy::DefaultPlugins;
use bevy::prelude::App;

use bevy_ball::enemy::EnemyPlugin;
use bevy_ball::player::PlayerPlugin;
use bevy_ball::score::ScorePlugin;
use bevy_ball::star::StarPlugin;
use bevy_ball::system::SystemPlugin;

fn main() {
    Debug::main();
}

struct Debug;

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

    #[allow(dead_code)]
    fn debug_x() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(SystemPlugin)
            .run();
    }
}
