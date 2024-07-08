use bevy::app::App;
use bevy::prelude::{Plugin, Startup};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu);
    }
}

fn main_menu() {
    println!("You are on the main menu page");
}
