use bevy::app::{App, AppExit, Update};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::prelude::{
    AlignItems, Bundle, ButtonBundle, Changed, Color, Commands, Component, default, Entity,
    EventWriter, FlexDirection, ImageBundle, in_state, Interaction, IntoSystemConfigs,
    JustifyContent, JustifyText, NextState, NodeBundle, OnEnter, OnExit, Plugin, Query, Res,
    ResMut, Style, Text, TextBundle, TextSection, TextStyle, UiImage, UiRect, Val, With,
};
use bevy::text::BreakLineOn;

use crate::ApplicationState;
use crate::asset_handler::AssetHandler;
use crate::ui::{BUTTON_COLOR_NORMAL, BUTTON_STYLE, UiButton};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(ApplicationState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (on_play_button_pressed, on_quit_button_pressed)
                    .run_if(in_state(ApplicationState::MainMenu)),
            );
    }
}

pub fn on_play_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<ApplicationState>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            next_state.set(ApplicationState::InGame);
        }
    }
}

pub fn on_quit_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut event_writer: EventWriter<AppExit>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            event_writer.send(AppExit);
        }
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct TitleSection;

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
                background_color: Color::DARK_GRAY.into(),
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
pub struct TitleSectionBundle {
    title_section: TitleSection,
    node_bundle: NodeBundle,
}

impl Default for TitleSectionBundle {
    fn default() -> Self {
        Self {
            title_section: TitleSection {},
            node_bundle: NodeBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(120.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayButtonBundle {
    ui_button: UiButton,
    play_button: PlayButton,
    button_bundle: ButtonBundle,
}

impl Default for PlayButtonBundle {
    fn default() -> Self {
        Self {
            play_button: PlayButton {},
            ui_button: UiButton {},
            button_bundle: ButtonBundle {
                background_color: BUTTON_COLOR_NORMAL.into(),
                style: BUTTON_STYLE,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct QuitButtonBundle {
    ui_button: UiButton,
    quit_button: QuitButton,
    button_bundle: ButtonBundle,
}

impl Default for QuitButtonBundle {
    fn default() -> Self {
        Self {
            ui_button: UiButton {},
            quit_button: QuitButton {},
            button_bundle: ButtonBundle {
                background_color: BUTTON_COLOR_NORMAL.into(),
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
            parent
                .spawn(TitleSectionBundle::default())
                .with_children(|parent| {
                    // image 1
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                        image: UiImage {
                            texture: asset_handler.player_texture.clone(),
                            ..default()
                        },
                        ..default()
                    });
                    // text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Ball Game",
                                TextStyle {
                                    font_size: 35.0,
                                    ..default()
                                },
                            )],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                            ..default()
                        },
                        ..default()
                    });
                    // image 2
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                        image: UiImage {
                            texture: asset_handler.enemy_texture.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
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
