use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Res, Transform, Window};

pub struct WindowHelper {}

impl WindowHelper {
    /// Given a &Window, return its center of the screen as Transform
    pub fn center(window: &Window) -> Transform {
        let middle_x = window.width() / 2.0;
        let middle_y = window.height() / 2.0;
        Transform::from_xyz(middle_x, middle_y, 0.0)
    }
}

pub struct MovementHelper {}

impl MovementHelper {
    /// Given a keyboard_input, return the final normalized direction as Vec3
    pub fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>) -> Vec3 {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        return direction;
    }
}
