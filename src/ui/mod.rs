use std::convert::Into;

use bevy::app::App;
use bevy::prelude::{
    AlignItems, BackgroundColor, BuildChildren, Bundle, ButtonBundle, Changed, Color, Commands,
    Component, DespawnRecursiveExt, Entity, FlexDirection, ImageBundle, in_state, Interaction,
    IntoSystemConfigs, JustifyContent, JustifyText, NodeBundle, OnEnter, OnExit, Or, Plugin, Query,
    Res, Style, Text, TextBundle, TextSection, TextStyle, UiImage, UiRect, Update, Val, With,
};
use bevy::text::BreakLineOn;
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

const BUTTON_COLOR_NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_COLOR_HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
const BUTTON_COLOR_PRESSED: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(ApplicationState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                button_color_change.run_if(in_state(ApplicationState::MainMenu)),
            );
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
    play_button: PlayButton,
    button_bundle: ButtonBundle,
}

impl Default for PlayButtonBundle {
    fn default() -> Self {
        Self {
            play_button: PlayButton {},
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
    quit_button: QuitButton,
    button_bundle: ButtonBundle,
}

impl Default for QuitButtonBundle {
    fn default() -> Self {
        Self {
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

pub fn button_color_change(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            Or<(With<PlayButton>, With<QuitButton>)>,
        ),
    >,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *background_color = BUTTON_COLOR_PRESSED.into(),
            Interaction::Hovered => *background_color = BUTTON_COLOR_HOVERED.into(),
            Interaction::None => *background_color = BUTTON_COLOR_NORMAL.into(),
        }
    }
}
