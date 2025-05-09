use bevy::prelude::{
    default, in_state, info, App, AssetServer, Bundle, ButtonInput, Commands, Component, Entity,
    Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, Name, OnEnter, OnExit, Plugin,
    Query, Res, ResMut, Sprite, SpriteBundle, Time, Transform, Update, Window, With, Without,
};
use bevy::window::PrimaryWindow;

use crate::asset_handler::AssetHandler;
use crate::game::enemy::{Enemy, ENEMY_SIZE};
use crate::game::score::Score;
use crate::game::star::{Star, STAR_SIZE};
use crate::game::{Confined, GameState, Size};
use crate::helpers::{AudioHelper, MovementHelper, WindowHelper};
use crate::{ApplicationState, ScheduleDespawn};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedWithEnemy>()
            .add_event::<CollidedWithStar>()
            .add_systems(OnEnter(ApplicationState::InGame), spawn_player)
            .add_systems(OnExit(ApplicationState::InGame), despawn_player)
            .add_systems(
                Update,
                movement
                    .chain()
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (on_hit_star_emit_collide_event, on_star_collide_despawn_star)
                    .chain()
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (
                    (
                        on_hit_enemy_emit_collide_event,
                        on_enemy_collide_despawn_player,
                    )
                        .chain(),
                    on_enemy_collide_play_game_over_sound,
                    on_star_collide_play_star_despawn_sound,
                    on_star_collide_event_add_score,
                )
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Event)]
pub struct CollidedWithEnemy {
    pub score: u32,
}

#[derive(Event)]
pub struct CollidedWithStar {
    pub star_entity: Entity,
}

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player {}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    player: Player,
    confined: Confined,
    size: Size,
    sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn at_center_of_the_screen(
        window: &Window,
        asset_handler: &Res<AssetHandler>,
    ) -> (Name, Player, Confined, Size, Sprite, Transform) {
        (
            Name::new("Player"),
            Player {},
            Confined {},
            Size { value: PLAYER_SIZE },
            Sprite::from_image(asset_handler.player_texture.clone()),
            WindowHelper::center(window),
        )
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_handler: Res<AssetHandler>,
) {
    if let Ok(window) = window_query.get_single() {
        commands.spawn(PlayerBundle::at_center_of_the_screen(
            window,
            &asset_handler,
        ));
    }
}

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let movement_direction = MovementHelper::handle_input(keyboard_input);
        transform.translation += movement_direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn on_hit_enemy_emit_collide_event(
    mut event_writer: EventWriter<CollidedWithEnemy>,
    score: Option<Res<Score>>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let is_collided = MovementHelper::is_collided(
                PLAYER_SIZE,
                player_transform.translation,
                ENEMY_SIZE,
                enemy_transform.translation,
            );

            if is_collided {
                info!("Event Writer: CollidedWithEnemy");
                if let Some(score) = &score {
                    event_writer.send(CollidedWithEnemy { score: score.value });
                } else {
                    event_writer.send(CollidedWithEnemy { score: 0 });
                }
            }
        }
    }
}

pub fn on_hit_star_emit_collide_event(
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), (With<Star>, Without<ScheduleDespawn>)>,
    mut event_writer: EventWriter<CollidedWithStar>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let is_collided = MovementHelper::is_collided(
                PLAYER_SIZE,
                player_transform.translation,
                STAR_SIZE,
                star_transform.translation,
            );

            if is_collided {
                event_writer.send(CollidedWithStar { star_entity });
            }
        }
    }
}

pub fn on_enemy_collide_despawn_player(
    mut commands: Commands,
    mut event_reader: EventReader<CollidedWithEnemy>,
    query: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = query.get_single() {
        for _event in event_reader.read() {
            commands
                .entity(player_entity)
                .insert(ScheduleDespawn::default());
        }
    }
}

pub fn on_enemy_collide_play_game_over_sound(
    mut commands: Commands,
    mut event_reader: EventReader<CollidedWithEnemy>,
    asset_server: Res<AssetServer>,
) {
    for _event in event_reader.read() {
        commands.spawn(AudioHelper::play_game_over_sound(&asset_server));
    }
}

pub fn on_star_collide_despawn_star(
    mut commands: Commands,
    mut event_reader: EventReader<CollidedWithStar>,
) {
    for event in event_reader.read() {
        let star_entity = event.star_entity;
        if let Some(mut entity_commands) = commands.get_entity(star_entity) {
            entity_commands.insert(ScheduleDespawn::default());
        }
    }
}

pub fn on_star_collide_play_star_despawn_sound(
    mut commands: Commands,
    mut event_reader: EventReader<CollidedWithStar>,
    asset_server: Res<AssetServer>,
) {
    for _event in event_reader.read() {
        commands.spawn(AudioHelper::play_obtain_star_sound(&asset_server));
    }
}

pub fn on_star_collide_event_add_score(
    mut score: Option<ResMut<Score>>,
    mut event_reader: EventReader<CollidedWithStar>,
) {
    let Some(score) = &mut score else {
        return;
    };

    for _event in event_reader.read() {
        score.value += 1;
    }
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = query.get_single() {
        commands.entity(player_entity).insert(ScheduleDespawn {});
    }
}
