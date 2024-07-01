use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Res, Transform, Window};
use rand::random;

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

    pub fn confine(window: &Window, unit_translation: Vec3, unit_size: f32) -> Vec3 {
        let half_unit_size = unit_size / 2.0;
        let x_min = 0.0 + half_unit_size;
        let x_max = window.width() - half_unit_size;
        let y_min = 0.0 + half_unit_size;
        let y_max = window.height() - half_unit_size;

        let mut new_translation = unit_translation;

        // bound x
        if new_translation.x < x_min {
            new_translation.x = x_min;
        } else if new_translation.x > x_max {
            new_translation.x = x_max;
        }

        // bound y
        if new_translation.y < y_min {
            new_translation.y = y_min;
        } else if new_translation.y > y_max {
            new_translation.y = y_max;
        }

        new_translation
    }
}

pub struct RandomHelper {}

impl RandomHelper {
    pub fn random_f32() -> f32 {
        random::<f32>()
    }
}

pub struct SoundHelper {}

impl SoundHelper {
    pub fn bounce_sound() -> String {
        return if RandomHelper::random_f32() < 0.5 {
            "audio/pluck_001.ogg".to_string()
        } else {
            "audio/pluck_002.ogg".to_string()
        };
    }

    pub fn obtain_star_sound() -> String {
        "audio/laserLarge_000.ogg".to_string()
    }

    pub fn game_over_sound() -> String {
        "audio/explosionCrunch_000.ogg".to_string()
    }
}
