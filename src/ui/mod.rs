use std::convert::Into;

use bevy::app::App;
use bevy::prelude::{
    AlignItems, BackgroundColor, Changed, Color, Component, Interaction, IntoSystemConfigs,
    JustifyContent, Node, Plugin, Query, Update, Val, With,
};

use crate::ui::game_over_menu::GameOverMenuPlugin;
use crate::ui::hud_menu::InGameHUDPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::pause_menu::PauseMenuPlugin;

mod game_over_menu;
mod hud_menu;
mod main_menu;
mod pause_menu;

const BUTTON_STYLE: Node = {
    let mut style = Node::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

const BUTTON_COLOR_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_COLOR_HOVERED: Color = Color::srgb(0.35, 0.35, 0.35);
const BUTTON_COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin)
            .add_plugins(InGameHUDPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(GameOverMenuPlugin)
            .add_systems(Update, handle_button_color_change.run_if(ui_button_present));
    }
}

#[derive(Component)]
pub struct UIButton;

pub fn handle_button_color_change(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<UIButton>)>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *background_color = BUTTON_COLOR_PRESSED.into(),
            Interaction::Hovered => *background_color = BUTTON_COLOR_HOVERED.into(),
            Interaction::None => *background_color = BUTTON_COLOR_NORMAL.into(),
        }
    }
}

pub fn ui_button_present(query: Query<(), (Changed<Interaction>, With<UIButton>)>) -> bool {
    !query.is_empty()
}
