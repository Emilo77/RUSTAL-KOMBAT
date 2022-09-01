use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

pub use physics::*;
pub use audio::*;
pub use combat::*;
pub use map::*;
pub use camera::*;
pub use player::*;
pub use utils::*;

use super::AppState;

mod audio;
mod combat;
mod map;
mod player;
mod utils;
mod camera;
mod physics;
mod healthbars;

#[derive(Component)]
pub struct PhantomEntity;

#[derive(Component)]
pub struct GameTextures {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,

    pub sword: Handle<Image>,
    pub sword2: Handle<Image>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(back_to_menu))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_all));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTextures {
        player_left: asset_server.load("images/samurai1.png"),
        player_right: asset_server.load("images/samurai2.png"),
        sword: asset_server.load("images/sword.png"),
        sword2: asset_server.load("images/sword2.png"),
    });
    commands
        .spawn_bundle(SpriteBundle { ..default() })
        .insert(PhantomEntity);

}

fn back_to_menu(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}

fn cleanup_all(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
