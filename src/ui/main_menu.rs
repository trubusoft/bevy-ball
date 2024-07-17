use bevy::app::{App, AppExit, Update};
use bevy::prelude::{
    AlignItems, BuildChildren, ButtonBundle, Changed, Color, Commands, Component, default,
    DespawnRecursiveExt, Entity, EventWriter, FlexDirection, ImageBundle, in_state, Interaction,
    IntoSystemConfigs, JustifyContent, JustifyText, Name, NextState, NodeBundle, OnEnter, OnExit,
    Plugin, Query, Res, ResMut, Style, Text, TextBundle, TextSection, TextStyle, UiImage, UiRect,
    Val, With,
};
use bevy::text::BreakLineOn;

use crate::ApplicationState;
use crate::asset_handler::AssetHandler;
use crate::game::GameState;
use crate::ui::UIButton;

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
    mut application_state: ResMut<NextState<ApplicationState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            application_state.set(ApplicationState::InGame);
            game_state.set(GameState::Running);
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

pub fn spawn_main_menu(mut commands: Commands, asset_handler: Res<AssetHandler>) {
    build_main_menu(&mut commands, &asset_handler);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
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

const TITLE_IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub fn build_main_menu(commands: &mut Commands, asset_handler: &Res<AssetHandler>) -> Entity {
    commands
        .spawn((
            Name::new("Main Menu Screen"),
            MainMenu {},
            NodeBundle {
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
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Title Section"),
                    TitleSection {},
                    NodeBundle {
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Left image on title section"),
                        ImageBundle {
                            style: TITLE_IMAGE_STYLE,
                            image: UiImage {
                                texture: asset_handler.player_texture.clone(),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Name::new("Game title text"),
                        TextBundle {
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
                        },
                    ));
                    parent.spawn((
                        Name::new("Right image on title section"),
                        ImageBundle {
                            style: TITLE_IMAGE_STYLE,
                            image: UiImage {
                                texture: asset_handler.enemy_texture.clone(),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    Name::new("Play Button"),
                    PlayButton {},
                    UIButton {},
                    ButtonBundle {
                        background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                        style: crate::ui::BUTTON_STYLE,
                        ..default()
                    },
                ))
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
            parent
                .spawn((
                    Name::new("Quit Button"),
                    QuitButton {},
                    UIButton {},
                    ButtonBundle {
                        background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                        style: crate::ui::BUTTON_STYLE,
                        ..default()
                    },
                ))
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
