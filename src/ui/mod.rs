use bevy::app::App;
use bevy::prelude::{
    BuildChildren, Bundle, Color, Commands, Component, DespawnRecursiveExt, Entity, NodeBundle,
    OnEnter, OnExit, Plugin, Query, Res, Style, Val, With,
};
use bevy::utils::default;

use crate::ApplicationState;
use crate::asset_handler::AssetHandler;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(ApplicationState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Bundle)]
pub struct MainMenuBundle {
    main_menu: MainMenu,
    node_bundle: NodeBundle,
}

impl Default for MainMenuBundle {
    fn default() -> Self {
        Self {
            main_menu: MainMenu {},
            node_bundle: NodeBundle {
                background_color: Color::SEA_GREEN.into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_handler: Res<AssetHandler>) {
    build_main_menu(&mut commands, &asset_handler);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_handler: &Res<AssetHandler>) -> Entity {
    commands.spawn(MainMenuBundle::default()).id()
}
