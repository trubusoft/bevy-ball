use bevy::app::AppExit;
use bevy::DefaultPlugins;
use bevy::prelude::{App, ButtonInput, EventWriter, KeyCode, Res, Update};

use bevy_ball::camera::CameraPlugin;
use bevy_ball::enemy::EnemyPlugin;
use bevy_ball::events::GameOver;
use bevy_ball::player::PlayerPlugin;
use bevy_ball::score::ScorePlugin;
use bevy_ball::star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(ScorePlugin)
        .add_systems(Update, exit_on_escape)
        .add_event::<GameOver>()
        .run();
}

fn exit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writter: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writter.send(AppExit);
    }
}
