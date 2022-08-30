use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

pub use audio::*;
pub use combat::*;
pub use components::*;
pub use map::*;
pub use camera::*;
pub use player::*;
pub use utils::*;

use super::AppState;

mod audio;
mod combat;
mod components;
mod map;
mod player;
mod utils;
mod camera;

#[derive(Component)]
pub struct PhantomEntity;

pub struct GameTextures {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,

    pub sword: Handle<Image>,
    pub bonus: Handle<Image>,

    pub floor: Handle<Image>,
    pub background: Handle<Image>,
    pub clouds: Handle<Image>,
    pub tree: Handle<Image>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_all));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTextures {
        player_left: asset_server.load("images/samurai1.png"),
        player_right: asset_server.load("images/samurai1.png"),
        sword: asset_server.load("images/samurai1.png"), //todo
        bonus: asset_server.load("images/samurai1.png"), //todo
        floor: asset_server.load("images/samurai1.png"), //todo
        background: asset_server.load("images/samurai1.png"), //todo
        clouds: asset_server.load("images/samurai1.png"), //todo
        tree: asset_server.load("images/samurai1.png"), //todo
    });
    commands
        .spawn_bundle(SpriteBundle { ..default() })
        .insert(PhantomEntity);

}

fn cleanup_all(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
