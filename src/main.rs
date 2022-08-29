use bevy::prelude::*;
use bevy::window::PresentMode;

use gameplay::GamePlugin;
use menu::MenuPlugin;

use crate::gameplay::{
    OwnAudioPlugin, CombatPlugin, PowerupPlugin, PlayerPlugin, MapPlugin,
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
            title: "Rustal combat".to_string(),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::Fifo,
            decorations: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::GOLD))
        .add_state(AppState::MainMenu)
        .add_plugin(GamePlugin)
        .add_plugin(OwnAudioPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(PowerupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(MenuPlugin)
        // .add_plugin(AiPlugin)
        .run();
}
