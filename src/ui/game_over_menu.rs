use bevy::app::AppExit;
use bevy::prelude::{
    default, in_state, AlignItems, App, BuildChildren, Button, Changed, ChildBuild, Color,
    Commands, Component, DespawnRecursiveExt, Display, Entity, EventWriter, FlexDirection,
    Interaction, IntoSystemConfigs, JustifyContent, JustifyText, NextState, Node, OnEnter, OnExit,
    Plugin, PositionType, Query, Res, ResMut, Text, TextColor, TextFont, TextLayout, Update, Val,
    With,
};

use crate::game::high_score::HighScore;
use crate::game::GameState;
use crate::ui::UIButton;
use crate::ApplicationState;

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
            event_writer.send(AppExit::Success);
        }
    }
}

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub const GAME_OVER_MENU_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex; // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const GAME_OVER_MENU_CONTAINER_STYLE: Node = {
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

pub const BUTTON_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

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

pub fn spawn_game_over_menu(mut commands: Commands, high_score: Option<Res<HighScore>>) {
    let final_score: i32;
    if let Some(high_score) = &high_score {
        let last_value = high_score.scores.last();
        if last_value.is_some() {
            final_score = last_value.unwrap().1 as i32;
        } else {
            final_score = 0;
        }
    } else {
        final_score = 0;
    }

    build_game_over_menu(&mut commands, final_score);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn build_game_over_menu(commands: &mut Commands, final_score: i32) -> Entity {
    let game_over_menu_entity = commands
        .spawn((GAME_OVER_MENU_STYLE, GameOverMenu {}))
        .with_children(|parent| {
            parent
                .spawn(GAME_OVER_MENU_CONTAINER_STYLE)
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Game Over"),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    // Final Score Text
                    parent.spawn((
                        Text::new(format!("Final Score: {}", final_score.to_string())),
                        TextFont {
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout::new_with_justify(JustifyText::Center),
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            Button {},
                            // ButtonBundle {
                            //     style: BUTTON_STYLE,
                            //     background_color: NORMAL_BUTTON.into(),
                            //     ..default()
                            // },
                            RestartButton {},
                            UIButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Restart"),
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
                            // ButtonBundle {
                            //     style: BUTTON_STYLE,
                            //     background_color: NORMAL_BUTTON.into(),
                            //     ..default()
                            // },
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
                            // ButtonBundle {
                            //     style: BUTTON_STYLE,
                            //     background_color: NORMAL_BUTTON.into(),
                            //     ..default()
                            // },
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

    game_over_menu_entity
}
