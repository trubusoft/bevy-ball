use bevy::app::App;
use bevy::prelude::{
    AlignItems, BuildChildren, Bundle, ButtonBundle, Color, Commands, Component,
    DespawnRecursiveExt, Entity, FlexDirection, JustifyContent, JustifyText, NodeBundle, OnEnter,
    OnExit, Plugin, Query, Res, Style, Text, TextBundle, TextSection, TextStyle, Val, With,
};
use bevy::utils::default;

use crate::ApplicationState;
use crate::asset_handler::AssetHandler;

const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(ApplicationState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

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
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct TitleBundle {
    title: Title,
    text_bundle: TextBundle,
}

impl Default for TitleBundle {
    fn default() -> Self {
        Self {
            title: Title {},
            text_bundle: TextBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub struct PlayButtonBundle {
    play_button: PlayButton,
    button_bundle: ButtonBundle,
}

impl Default for PlayButtonBundle {
    fn default() -> Self {
        Self {
            play_button: PlayButton {},
            button_bundle: ButtonBundle {
                background_color: Color::YELLOW_GREEN.into(),
                style: BUTTON_STYLE,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct QuitButtonBundle {
    quit_button: QuitButton,
    button_bundle: ButtonBundle,
}

impl Default for QuitButtonBundle {
    fn default() -> Self {
        Self {
            quit_button: QuitButton {},
            button_bundle: ButtonBundle {
                background_color: Color::ORANGE_RED.into(),
                style: BUTTON_STYLE,
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
    commands
        .spawn(MainMenuBundle::default())
        .with_children(|parent| {
            // title
            parent.spawn(TitleBundle::default());
            // play button
            parent
                .spawn(PlayButtonBundle::default())
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font_size: 35.0,
                                    ..default()
                                },
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // quit button
            parent
                .spawn(QuitButtonBundle::default())
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                TextStyle {
                                    font_size: 35.0,
                                    ..default()
                                },
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id()
}
