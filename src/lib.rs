use bevy::prelude::{Transform, Window};

pub struct WindowHelper {}

impl WindowHelper {
    /// Given a &Window, return its center of the screen as Transform
    pub fn center(window: &Window) -> Transform {
        let middle_x = window.width() / 2.0;
        let middle_y = window.height() / 2.0;
        Transform::from_xyz(middle_x, middle_y, 0.0)
    }
}
