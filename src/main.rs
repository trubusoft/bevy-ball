use bevy::app::AppExit;
use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::{
    App, AssetServer, AudioBundle, ButtonInput, Camera2dBundle, Commands, Component, default,
    DetectChanges, Entity, Event, EventReader, EventWriter, KeyCode, PlaybackSettings, Query, Res,
    ResMut, Resource, SpriteBundle, Startup, Time, Timer, TimerMode, Transform, Update, Window,
    With,
};
use bevy::window::PrimaryWindow;

use bevy_ball::{MovementHelper, RandomHelper, SoundHelper, WindowHelper};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, player_hit_enemy)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_hit_star)
        .init_resource::<Score>()
        .add_systems(Update, print_score_on_change)
        .init_resource::<StarSpawnTimer>()
        .add_systems(Update, tick_spawn_star_timer)
        .add_systems(Update, spawn_stars_overtime)
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

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: WindowHelper::center(window),
        ..default()
    });
}

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Player {},
        SpriteBundle {
            transform: WindowHelper::center(window),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
    ));
}

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let movement_direction = MovementHelper::handle_input(keyboard_input);
        transform.translation += movement_direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let confined_translation =
            MovementHelper::confine(window, player_transform.translation, PLAYER_SIZE);
        player_transform.translation = confined_translation;
    }
}

const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SIZE: f32 = 64.0;

#[derive(Component)]
struct Enemy {
    pub direction: Vec3,
}

impl Enemy {
    fn randomize_direction() -> Vec3 {
        Vec3::new(RandomHelper::random_f32(), RandomHelper::random_f32(), 0.0).normalize()
    }
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
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

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        let enemy_direction = enemy.direction;
        enemy_transform.translation += enemy_direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let half_unit_size = ENEMY_SIZE / 2.0;
        let x_min = 0.0 + half_unit_size;
        let x_max = window.width() - half_unit_size;
        let y_min = 0.0 + half_unit_size;
        let y_max = window.height() - half_unit_size;

        let new_translation = enemy_transform.translation;
        let mut is_direction_changed: bool = false;

        if new_translation.x <= x_min || new_translation.x >= x_max {
            enemy.direction.x *= -1.0;
            is_direction_changed = true;
        }
        if new_translation.y <= y_min || new_translation.y >= y_max {
            enemy.direction.y *= -1.0;
            is_direction_changed = true;
        }

        if is_direction_changed {
            commands.spawn(AudioBundle {
                source: asset_server.load(SoundHelper::bounce_sound()),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let confined_translation =
            MovementHelper::confine(window, enemy_transform.translation, ENEMY_SIZE);
        enemy_transform.translation = confined_translation;
    }
}

fn player_hit_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut game_over_event_writter: EventWriter<GameOver>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            let actual_distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if actual_distance <= (player_radius + enemy_radius) {
                commands.spawn(AudioBundle {
                    source: asset_server.load(SoundHelper::game_over_sound()),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(player_entity).despawn();
                game_over_event_writter.send(GameOver { score: score.value });
            }
        }
    }
}

const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;

#[derive(Component)]
struct Star {}

fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.get_single() {
        for _ in 0..NUMBER_OF_STARS {
            let random_x = RandomHelper::random_f32() * window.width();
            let random_y = RandomHelper::random_f32() * window.height();

            commands.spawn((
                Star {},
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
            ));
        }
    }
}

fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            let actual_distance = player_transform
                .translation
                .distance(star_transform.translation);

            if actual_distance <= (player_radius + star_radius) {
                commands.spawn(AudioBundle {
                    source: asset_server.load(SoundHelper::obtain_star_sound()),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
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

const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

fn tick_spawn_star_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_overtime(
    mut commands: Commands,
    star_spawn_timer: Res<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.just_finished() {
        if let Ok(window) = window_query.get_single() {
            let random_x = RandomHelper::random_f32() * window.width();
            let random_y = RandomHelper::random_f32() * window.height();

            commands.spawn((
                Star {},
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
            ));
        }
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

#[derive(Event)]
struct GameOver {
    pub score: u32,
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
