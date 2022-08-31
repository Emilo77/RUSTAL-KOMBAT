use bevy::prelude::*;
use bevy::window::PresentMode;

use gameplay::GamePlugin;
use menu::MenuPlugin;

use crate::gameplay::{
    OwnAudioPlugin, CombatPlugin, PowerupPlugin, PlayerPlugin, MapPlugin, ScoreboardPlugin,
    OwnCameraPlugin
};

mod gameplay;
mod menu;

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    MainMenu,
    EndMenu,
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Rustal Combat".to_string(),
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            resizable: false,
            present_mode: PresentMode::Fifo,
            decorations: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_state(AppState::MainMenu)
        .add_plugin(MenuPlugin)
        .add_plugin(OwnAudioPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(OwnCameraPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ScoreboardPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
