use bevy::app::AppExit;
use bevy::DefaultPlugins;
use bevy::prelude::{
    App, AssetServer, ButtonInput, Commands, default, EventWriter, KeyCode, Query, Res, ResMut,
    Resource, SpriteBundle, Time, Timer, TimerMode, Transform, Update, Window, With,
};
use bevy::window::PrimaryWindow;

use bevy_ball::camera::CameraPlugin;
use bevy_ball::enemy::components::Enemy;
use bevy_ball::enemy::EnemyPlugin;
use bevy_ball::events::GameOver;
use bevy_ball::helpers::RandomHelper;
use bevy_ball::player::PlayerPlugin;
use bevy_ball::score::ScorePlugin;
use bevy_ball::star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(ScorePlugin)
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Update, tick_spawn_enemy_timer)
        .add_systems(Update, spawn_enemy_overtime)
        .add_systems(Update, exit_on_escape)
        .add_event::<GameOver>()
        .run();
}

const ENEMY_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

fn tick_spawn_enemy_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_enemy_overtime(
    mut commands: Commands,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.just_finished() {
        let window = window_query.get_single().unwrap();
        let random_x = RandomHelper::random_f32() * window.width();
        let random_y = RandomHelper::random_f32() * window.height();

        commands.spawn((
            Enemy {
                direction: Enemy::randomize_direction(),
            },
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}

fn exit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writter: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writter.send(AppExit);
    }
}
