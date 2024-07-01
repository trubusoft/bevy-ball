use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{EventWriter, KeyCode, Res};

pub fn exit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writter: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writter.send(AppExit);
    }
}
