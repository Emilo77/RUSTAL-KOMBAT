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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    MainMenu,
    EndMenu,
    StopMenu,
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Rustal Combat".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            // present_mode: PresentMode::Fifo,
            // decorations: true,
            ..Default::default()
        })
        // .insert_resource(ClearColor(Color::GOLD))
        .add_plugin(MapPlugin)
        .add_plugin(OwnCameraPlugin)
        .add_state(AppState::MainMenu)
        .add_plugin(GamePlugin)
        .add_plugin(ScoreboardPlugin)
        .add_plugin(OwnAudioPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(PowerupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MenuPlugin)
        // .add_plugin(AiPlugin)
        .run();
}
