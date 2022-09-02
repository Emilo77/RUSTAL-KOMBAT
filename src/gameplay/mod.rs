use bevy::prelude::*;

pub use audio::*;
pub use combat::*;
pub use map::*;
pub use camera::*;
pub use player::*;
pub use utils::*;
pub use assets_handling::*;

use super::AppState;

mod audio;
mod combat;
mod map;
mod player;
mod utils;
mod camera;
mod boards;
mod assets_handling;
mod animation;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_system_set(SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup_all))
            .add_plugin(OwnAudioPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(OwnCameraPlugin)
            .add_plugin(PlayerPlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTextures::load(asset_server));
}

fn cleanup_all(mut commands: Commands, query: Query<(Entity, &mut Player)>) {
    for (entity, _player) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
