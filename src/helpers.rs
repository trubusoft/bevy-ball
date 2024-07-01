use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
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

    pub fn is_collided(
        first_size: f32,
        first_transform: Vec3,
        second_size: f32,
        second_transform: Vec3,
    ) -> bool {
        let first_length = first_size / 2.0;
        let second_length = second_size / 2.0;
        let distance_between_them = first_transform.distance(second_transform);

        distance_between_them <= (first_length + second_length)
    }
}

pub struct RandomHelper {}

impl RandomHelper {
    pub fn random_f32() -> f32 {
        random::<f32>()
    }
}

pub struct AudioHelper {}

impl AudioHelper {
    fn play_once(asset_server: &Res<AssetServer>, file_name: String) -> AudioBundle {
        AudioBundle {
            source: asset_server.load(file_name),
            settings: PlaybackSettings::DESPAWN,
        }
    }

    pub fn play_bounce_sound(asset_server: &Res<AssetServer>) -> AudioBundle {
        Self::play_once(&asset_server, AudioHelper::bounce_sound())
    }

    pub fn play_obtain_star_sound(asset_server: &Res<AssetServer>) -> AudioBundle {
        Self::play_once(&asset_server, AudioHelper::obtain_star_sound())
    }

    pub fn play_game_over_sound(asset_server: &Res<AssetServer>) -> AudioBundle {
        Self::play_once(&asset_server, AudioHelper::game_over_sound())
    }

    fn bounce_sound() -> String {
        return if RandomHelper::random_f32() < 0.5 {
            "audio/pluck_001.ogg".to_string()
        } else {
            "audio/pluck_002.ogg".to_string()
        };
    }

    fn obtain_star_sound() -> String {
        "audio/laserLarge_000.ogg".to_string()
    }

    fn game_over_sound() -> String {
        "audio/explosionCrunch_000.ogg".to_string()
    }
}

pub struct SpriteHelper {}

impl SpriteHelper {
    pub fn enemy_sprite() -> String {
        "sprites/ball_red_large.png".to_string()
    }
}
