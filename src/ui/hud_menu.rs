use bevy::prelude::{
    AlignItems, App, BuildChildren, Color, Commands, Component, default, DespawnRecursiveExt,
    DetectChanges, Display, Entity, FlexDirection, ImageBundle, in_state,
    IntoSystemConfigs, JustifyContent, JustifyText, NodeBundle, OnEnter, OnExit, Plugin, Query,
    Res, Style, Text, TextBundle, TextSection, TextStyle, UiImage, UiRect, Update, Val, With,
};

use crate::ApplicationState;
use crate::asset_handler::AssetHandler;
use crate::game::enemy::Enemy;
use crate::game::GameState;
use crate::game::score::Score;

pub struct InGameHUDPlugin;

impl Plugin for InGameHUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::InGame), spawn_hud)
            .add_systems(OnExit(ApplicationState::InGame), despawn_hud)
            .add_systems(
                Update,
                (update_score_text, update_enemy_text)
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

pub fn spawn_hud(mut commands: Commands, asset_handler: Res<AssetHandler>) {
    build_hud(&mut commands, &asset_handler);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", count.to_string());
    }
}

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const HUD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(15.0);
    style
};

pub const LHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.margin = UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0));
    style
};

pub const RHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.margin = UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0));
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.height = Val::Px(48.0);
    style.width = Val::Px(48.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub fn get_text_style() -> TextStyle {
    TextStyle {
        font_size: 64.0,
        color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    }
}

#[derive(Component)]
pub struct HUD {}

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component)]
pub struct EnemyText {}

pub fn build_hud(commands: &mut Commands, asset_handler: &Res<AssetHandler>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: LHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Star Image
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: UiImage {
                            texture: asset_handler.star_texture.clone(),
                            ..default()
                        },
                        ..default()
                    });
                    // Score Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style())],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });
            // RHS
            parent
                .spawn(NodeBundle {
                    style: RHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style())],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyText {},
                    ));
                    // Enemy Image
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: UiImage {
                            texture: asset_handler.enemy_texture.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    hud_entity
}
