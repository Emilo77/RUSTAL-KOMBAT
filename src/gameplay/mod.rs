use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

pub use audio::*;
pub use camera::*;
pub use combat::*;
pub use components::*;
pub use map::*;
pub use player::*;
pub use utils::*;

use super::AppState;

mod audio;
mod camera;
mod combat;
mod components;
mod map;
mod player;
mod utils;

pub struct GameTextures {
    pub player: Handle<Image>,
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
        // add things to your app here
    }
}

// impl Plugin for GamePlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_startup_system_to_stage(StartupStage::PreStartup, setup)
//             .add_system_set(
//                 SystemSet::on_update(AppState::InGame).with_system(back_to_main_menu_controls),
//             )
//             .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_all))
//             .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
//     }
// }
//
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // Add the game's entities to our world
//
//     commands
//         .spawn_bundle(SpriteBundle { ..default() })
//         .insert(PhantomEntity);
// }
