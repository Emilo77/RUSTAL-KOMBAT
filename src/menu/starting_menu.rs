use bevy::prelude::*;
use crate::AppState;
use crate::menu::{back_to_game, cleanup_menu, FAST_BLINKING, MAIN_IMAGE_POSITION, MAIN_IMAGE_SIZE};
use crate::menu::assets_handling::MenuTextures;
use crate::menu::utils::*;

pub struct StartingMenuPlugin;

impl Plugin for StartingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu)
            .with_system(setup_main_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu)
                .with_system(image_blinking)
                .with_system(back_to_game))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu)
                .with_system(cleanup_menu));
    }
}

fn setup_main_menu(mut commands: Commands, textures: Res<MenuTextures>) {
    spawn_main_image(&mut commands, textures.starting_text.clone(), true,
                     FAST_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);
}