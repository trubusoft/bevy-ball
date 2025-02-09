use bevy::DefaultPlugins;
use bevy::prelude::App;

use bevy_ball::ApplicationPlugin;
use bevy_ball::game::enemy::EnemyPlugin;
use bevy_ball::game::GamePlugin;
use bevy_ball::game::player::PlayerPlugin;
use bevy_ball::game::score::ScorePlugin;
use bevy_ball::game::star::StarPlugin;
use bevy_ball::ui::UIPlugin;

fn main() {
    Debug::main();
    // Debug::debug_application();
    // Debug::debug_ui();
    // Debug::debug_game();

    // GamePlugin
    // Debug::debug_player();
    // Debug::debug_player_star();
    // Debug::debug_player_star_score();
    // Debug::debug_enemy();
}

struct Debug;

#[allow(dead_code)]
impl Debug {
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(UIPlugin)
            .add_plugins(GamePlugin)
            .run();
    }

    fn debug_application() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .run();
    }

    fn debug_ui() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(UIPlugin)
            .run();
    }

    fn debug_game() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(GamePlugin)
            .run();
    }

    fn debug_player() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(PlayerPlugin)
            .run();
    }

    fn debug_player_star() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .run();
    }

    fn debug_player_star_score() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .run();
    }

    fn debug_enemy() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(ApplicationPlugin)
            .add_plugins(EnemyPlugin)
            .run();
    }
}
