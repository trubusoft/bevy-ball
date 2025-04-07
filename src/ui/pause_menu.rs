use bevy::app::{App, AppExit};
use bevy::prelude::{
    default, in_state, info, AlignItems, BuildChildren, Button, ButtonBundle, Changed, ChildBuild,
    Color, Commands, Component, DespawnRecursiveExt, Display, Entity, EventWriter, FlexDirection,
    Interaction, IntoSystemConfigs, JustifyContent, JustifyText, Name, NextState, Node, NodeBundle,
    OnEnter, OnExit, Plugin, PositionType, Query, ResMut, Text, TextBundle, TextColor, TextFont,
    TextLayout, Update, Val, With, ZIndex,
};

use crate::game::GameState;
use crate::ui::{UIButton, BUTTON_COLOR_NORMAL, BUTTON_STYLE};
use crate::ApplicationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Paused),
            spawn_pause_menu.run_if(in_state(ApplicationState::InGame)),
        )
        .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
        .add_systems(
            Update,
            (
                on_resume_button_pressed,
                on_main_menu_button_pressed,
                on_quit_button_pressed,
            )
                .run_if(in_state(ApplicationState::InGame))
                .run_if(in_state(GameState::Paused)),
        );
    }
}

pub fn on_resume_button_pressed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Running);
        }
    }
}

pub fn on_main_menu_button_pressed(
    mut application_state: ResMut<NextState<ApplicationState>>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            application_state.set(ApplicationState::MainMenu);
            game_state.set(GameState::Stop)
        }
    }
}

pub fn on_quit_button_pressed(
    mut event_writer: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            event_writer.send(AppExit::Success);
        }
    }
}

pub fn spawn_pause_menu(mut commands: Commands) {
    info!("Spawning Pause Menu");
    build_pause_menu(&mut commands);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        info!("Despawning Pause Menu");
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct PauseMenu {}

#[derive(Component)]
pub struct ResumeButton {}

#[derive(Component)]
pub struct MainMenuButton {}

#[derive(Component)]
pub struct QuitButton {}

pub const PAUSE_MENU_BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);

pub const PAUSE_MENU_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex; // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const PAUSE_MENU_CONTAINER_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(400.0);
    style.height = Val::Px(400.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub fn build_pause_menu(commands: &mut Commands) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            Name::new("Pause Menu"),
            PauseMenu {},
            PAUSE_MENU_STYLE,
            // z_index: ZIndex::Local(1), // UI Z-Index | https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PAUSE_MENU_CONTAINER_STYLE,
                    // background_color: crate::ui::pause_menu::PAUSE_MENU_BACKGROUND_COLOR.into(),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Pause menu"),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    // Resume Button
                    parent
                        .spawn((
                            Button {},
                            BUTTON_STYLE,
                            // background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                            ResumeButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Resume"),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            Button {},
                            BUTTON_STYLE,
                            // background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                            MainMenuButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Main Menu"),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                    // Quit Button
                    parent
                        .spawn((
                            Button {},
                            BUTTON_STYLE,
                            // background_color: crate::ui::BUTTON_COLOR_NORMAL.into(),
                            QuitButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                });
        })
        .id();

    pause_menu_entity
}
