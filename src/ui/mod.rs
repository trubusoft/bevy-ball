use std::convert::Into;

use bevy::app::App;
use bevy::prelude::{
    AlignItems, BackgroundColor, Changed, Color, Component, in_state, Interaction,
    IntoSystemConfigs, JustifyContent, Plugin, Query, Style, Update, Val, With,
};

use crate::ApplicationState;
use crate::ui::main_menu::MainMenuPlugin;

mod main_menu;

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
        app.add_plugins(MainMenuPlugin).add_systems(
            Update,
            button_color_change.run_if(in_state(ApplicationState::MainMenu)),
        );
    }
}

#[derive(Component)]
pub struct UiButton;

pub fn button_color_change(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<UiButton>)>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *background_color = BUTTON_COLOR_PRESSED.into(),
            Interaction::Hovered => *background_color = BUTTON_COLOR_HOVERED.into(),
            Interaction::None => *background_color = BUTTON_COLOR_NORMAL.into(),
        }
    }
}
