use bevy::prelude::States;

pub mod game;
pub mod helpers;
pub mod system;
pub mod ui;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
