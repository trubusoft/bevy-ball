use bevy::prelude::{
    default, in_state, AlignItems, App, BuildChildren, ChildBuild, Color, Commands, Component,
    DespawnRecursiveExt, DetectChanges, Display, Entity, FlexDirection, ImageNode,
    IntoSystemConfigs, JustifyContent, JustifyText, Node, OnEnter, OnExit, Plugin, Query, Res,
    Text, TextColor, TextFont, TextLayout, UiRect, Update, Val, With,
};

use crate::asset_handler::AssetHandler;
use crate::game::enemy::Enemy;
use crate::game::score::Score;
use crate::game::GameState;
use crate::ApplicationState;

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
            // text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        // text.sections[0].value = format!("{}", count.to_string());
    }
}

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const HUD_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(15.0);
    style
};

pub const LHS_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.margin = UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0));
    style
};

pub const RHS_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.margin = UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0));
    style
};

pub const IMAGE_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.height = Val::Px(48.0);
    style.width = Val::Px(48.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

#[derive(Component)]
pub struct HUD {}

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component)]
pub struct EnemyText {}

pub fn build_hud(commands: &mut Commands, asset_handler: &Res<AssetHandler>) -> Entity {
    let hud_entity = commands
        .spawn(HUD_STYLE)
        .with_children(|parent| {
            // LHS
            parent.spawn(LHS_STYLE).with_children(|parent| {
                // Star Image
                parent.spawn((
                    IMAGE_STYLE,
                    ImageNode::new(asset_handler.star_texture.clone()),
                ));
                // Score Text
                parent.spawn((
                    Text::new("0"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    TextLayout::new_with_justify(JustifyText::Center),
                    ScoreText {},
                ));
            });
            // RHS
            parent
                .spawn((
                    RHS_STYLE,
                    //background_color: crate::ui::hud_menu::BACKGROUND_COLOR.into(),
                ))
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        Text::new("0"),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        TextLayout::new_with_justify(JustifyText::Center),
                        EnemyText {},
                    ));
                    // Enemy Image
                    parent.spawn((
                        ImageNode::new(asset_handler.enemy_texture.clone()),
                        // style: crate::ui::hud_menu::IMAGE_STYLE,
                    ));
                });
        })
        .id();

    hud_entity
}
