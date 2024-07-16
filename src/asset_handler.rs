use bevy::prelude::{
    App, AssetServer, AudioSource, Handle, Image, Plugin, Res, ResMut, Resource, Startup,
};

#[derive(Resource, Debug, Default)]
pub struct AssetHandler {
    pub player_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
    pub star_texture: Handle<Image>,
    pub bounce_1_sound: Handle<AudioSource>,
    pub bounce_2_sound: Handle<AudioSource>,
    pub obtain_star_sound: Handle<AudioSource>,
    pub game_over_sound: Handle<AudioSource>,
}

pub struct AssetHandlerPlugin;
impl Plugin for AssetHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandler>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<AssetHandler>, asset_server: Res<AssetServer>) {
    scene_assets.player_texture = asset_server.load("sprites/ball_blue_large.png");
    scene_assets.enemy_texture = asset_server.load("sprites/ball_red_large.png");
    scene_assets.star_texture = asset_server.load("sprites/star.png");
    scene_assets.bounce_1_sound = asset_server.load("audio/pluck_001.ogg");
    scene_assets.bounce_2_sound = asset_server.load("audio/pluck_002.ogg");
    scene_assets.obtain_star_sound = asset_server.load("audio/laserLarge_000.ogg");
    scene_assets.game_over_sound = asset_server.load("audio/explosionCrunch_000.ogg");

    // *scene_assets = SceneAssets {
    //     player_texture: asset_server.load("sprites/ball_blue_large.png"),
    //     enemy_texture: asset_server.load("sprites/ball_red_large.png"),
    //     star_texture: asset_server.load("sprites/star.png"),
    //     bounce_1_sound: asset_server.load("audio/pluck_001.ogg"),
    //     bounce_2_sound: asset_server.load("audio/pluck_002.ogg"),
    //     obtain_star_sound: asset_server.load("audio/laserLarge_000.ogg"),
    //     game_over_sound: asset_server.load("audio/explosionCrunch_000.ogg"),
    // };
}
