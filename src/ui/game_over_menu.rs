use bevy::app::AppExit;
use bevy::prelude::{
    AlignItems, App, BuildChildren, ButtonBundle, Changed, Color, Commands, Component, default,
    DespawnRecursiveExt, Display, Entity, EventReader, EventWriter, FlexDirection, in_state,
    Interaction, IntoSystemConfigs, JustifyContent, JustifyText, NextState, NodeBundle, OnEnter,
    OnExit, Plugin, PositionType, Query, ResMut, Style, Text, TextBundle, TextSection, TextStyle,
    Update, Val, With, ZIndex,
};

use crate::ApplicationState;
use crate::game::GameState;
use crate::game::player::CollidedWithEnemy;
use crate::ui::UIButton;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(ApplicationState::GameOver), despawn_game_over_menu)
            .add_systems(
                Update,
                (
                    on_restart_button_pressed,
                    on_main_menu_button_pressed,
                    on_quit_button_pressed,
                    update_final_score_text,
                )
                    .run_if(in_state(ApplicationState::GameOver)),
            );
    }
}

pub fn on_restart_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut application_state: ResMut<NextState<ApplicationState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            application_state.set(ApplicationState::InGame);
            game_state.set(GameState::Running);
        }
    }
}

pub fn on_main_menu_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut application_state: ResMut<NextState<ApplicationState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            application_state.set(ApplicationState::MainMenu);
            game_state.set(GameState::Stop);
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

pub fn update_final_score_text(
    mut event_reader: EventReader<CollidedWithEnemy>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in event_reader.read() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score.to_string());
        }
    }
}

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub const GAME_OVER_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex; // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const GAME_OVER_MENU_CONTAINER_STYLE: Style = {
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

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn get_title_text_style() -> TextStyle {
    TextStyle {
        font_size: 64.0,
        color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    }
}

pub fn get_final_score_text_style() -> TextStyle {
    TextStyle {
        font_size: 48.0,
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

#[derive(Component)]
pub struct GameOverMenu {}

#[derive(Component)]
pub struct FinalScoreText {}

#[derive(Component)]
pub struct RestartButton {}

#[derive(Component)]
pub struct MainMenuButton {}

#[derive(Component)]
pub struct QuitButton {}

pub fn spawn_game_over_menu(mut commands: Commands) {
    build_game_over_menu(&mut commands);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn build_game_over_menu(commands: &mut Commands) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_MENU_STYLE,
                z_index: ZIndex::Local(2), // UI Z-Index | https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: GAME_OVER_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Game Over", get_title_text_style())],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Final Score Text
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Your final score was:",
                                    get_final_score_text_style(),
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            RestartButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Restart",
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
                                background_color: NORMAL_BUTTON.into(),
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
                                background_color: NORMAL_BUTTON.into(),
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

    game_over_menu_entity
}
