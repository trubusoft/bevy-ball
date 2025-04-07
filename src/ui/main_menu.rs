use bevy::app::{App, AppExit, Update};
use bevy::color::Color;
use bevy::prelude::{
    default, in_state, AlignItems, BuildChildren, Button, Changed, ChildBuild, Commands, Component,
    DespawnRecursiveExt, Entity, EventWriter, FlexDirection, ImageNode, Interaction,
    IntoSystemConfigs, JustifyContent, JustifyText, LineBreak, Name, NextState, Node, OnEnter,
    OnExit, Plugin, Query, Res, ResMut, Text, TextColor, TextFont, TextLayout, UiRect, Val, With,
};

use crate::asset_handler::AssetHandler;
use crate::game::GameState;
use crate::ui::UIButton;
use crate::ApplicationState;

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
            event_writer.send(AppExit::Success);
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

const TITLE_IMAGE_STYLE: Node = {
    let mut style = Node::DEFAULT;
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            },
            // background_color: Color::WHITE.into(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Title Section"),
                    TitleSection {},
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Left image on title section"),
                        ImageNode::new(asset_handler.player_texture.clone()),
                        TITLE_IMAGE_STYLE,
                    ));
                    parent.spawn((
                        Name::new("Game title text"),
                        Text::new("Bevy Ball Game"),
                        TextFont {
                            font_size: 35.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout {
                            justify: JustifyText::Center,
                            linebreak: LineBreak::NoWrap,
                        },
                    ));
                    parent.spawn((
                        Name::new("Right image on title section"),
                        ImageNode::new(asset_handler.enemy_texture.clone()),
                        TITLE_IMAGE_STYLE,
                    ));
                });
            parent
                .spawn((
                    Name::new("Play Button"),
                    PlayButton {},
                    UIButton {},
                    Button {},
                    crate::ui::BUTTON_STYLE,
                    // background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 35.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            parent
                .spawn((
                    Name::new("Quit Button"),
                    QuitButton {},
                    UIButton {},
                    Button {},
                    crate::ui::BUTTON_STYLE,
                    // background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextFont {
                            font_size: 35.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        })
        .id()
}
