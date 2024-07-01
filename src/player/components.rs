use bevy::asset::AssetServer;
use bevy::prelude::{Component, default, Res, SpriteBundle, Window};

use crate::helpers::{SpriteHelper, WindowHelper};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn at_center_of_the_screen(
        window: &Window,
        asset_server: &Res<AssetServer>,
    ) -> (Self, SpriteBundle) {
        (
            Player {},
            SpriteBundle {
                transform: WindowHelper::center(window),
                texture: asset_server.load(SpriteHelper::player_sprite()),
                ..default()
            },
        )
    }
}
