use bevy::app::AppExit;
use bevy::DefaultPlugins;
use bevy::prelude::{
    App, AssetServer, ButtonInput, Commands, default, DetectChanges, EventReader, EventWriter,
    KeyCode, Query, Res, ResMut, Resource, SpriteBundle, Time, Timer, TimerMode, Transform, Update,
    Window, With,
};
use bevy::window::PrimaryWindow;

use bevy_ball::camera::CameraPlugin;
use bevy_ball::enemy::components::Enemy;
use bevy_ball::enemy::EnemyPlugin;
use bevy_ball::events::GameOver;
use bevy_ball::helpers::RandomHelper;
use bevy_ball::player::PlayerPlugin;
use bevy_ball::star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .init_resource::<Score>()
        .add_systems(Update, print_score_on_change)
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Update, tick_spawn_enemy_timer)
        .add_systems(Update, spawn_enemy_overtime)
        .add_systems(Update, exit_on_escape)
        .add_event::<GameOver>()
        .add_systems(Update, handle_game_over_event)
        .init_resource::<HighScore>()
        .add_systems(Update, update_high_score)
        .add_systems(Update, print_high_score_on_change)
        .run();
}

#[derive(Resource)]
struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

fn print_score_on_change(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value);
    }
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

fn handle_game_over_event(mut event_reader: EventReader<GameOver>) {
    for event in event_reader.read() {
        println!("Your final score is: {}", event.score)
    }
}

#[derive(Resource, Debug)]
struct HighScore {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScore {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

fn update_high_score(mut event_reader: EventReader<GameOver>, mut high_score: ResMut<HighScore>) {
    for event in event_reader.read() {
        let last_score = event.score;
        high_score.scores.push(("Player".to_string(), last_score));
    }
}

fn print_high_score_on_change(high_score: Res<HighScore>) {
    if high_score.is_changed() {
        println!("High Score updated: {:?}", high_score);
    }
}
