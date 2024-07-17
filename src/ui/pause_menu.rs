use bevy::app::{App, AppExit};
use bevy::prelude::{
    AlignItems, BuildChildren, ButtonBundle, Changed, Color, Commands, Component, default, DespawnRecursiveExt,
    Display, Entity, EventWriter, FlexDirection, in_state, info, Interaction,
    IntoSystemConfigs, JustifyContent, JustifyText, Name, NextState, NodeBundle, OnEnter, OnExit,
    Plugin, PositionType, Query, ResMut, Style, Text, TextBundle, TextSection, TextStyle, Update,
    Val, With, ZIndex,
};

use crate::ApplicationState;
use crate::game::GameState;
use crate::ui::{BUTTON_COLOR_NORMAL, BUTTON_STYLE, UIButton};

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
            event_writer.send(AppExit);
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

pub const PAUSE_MENU_BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const PAUSE_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex; // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const PAUSE_MENU_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
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

pub fn get_title_text_style() -> TextStyle {
    TextStyle {
        font_size: 64.0,
        color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    }
}

pub fn get_button_text_style() -> TextStyle {
    TextStyle {
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    }
}

pub fn build_pause_menu(commands: &mut Commands) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            Name::new("Pause Menu"),
            PauseMenu {},
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                z_index: ZIndex::Local(1), // UI Z-Index | https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: PAUSE_MENU_CONTAINER_STYLE,
                    background_color: PAUSE_MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Pause Menu", get_title_text_style())],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Resume Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BUTTON_COLOR_NORMAL.into(),
                                ..default()
                            },
                            ResumeButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
                                        get_button_text_style(),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BUTTON_COLOR_NORMAL.into(),
                                ..default()
                            },
                            MainMenuButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_button_text_style(),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Quit Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BUTTON_COLOR_NORMAL.into(),
                                ..default()
                            },
                            QuitButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit",
                                        get_button_text_style(),
                                    )],
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    pause_menu_entity
}
