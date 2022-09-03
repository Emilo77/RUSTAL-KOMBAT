use bevy::prelude::*;
use crate::AppState;
use crate::menu::{back_to_game, BELOW_IMAGE_POSITION, BELOW_IMAGE_SIZE,
                  cleanup_menu, MAIN_IMAGE_POSITION, MAIN_IMAGE_SIZE,
                  SLOW_BLINKING, SUPER_SLOW_BLINKING};
use crate::menu::assets_handling::MenuTextures;
use crate::menu::utils::{image_blinking, spawn_main_image};


pub struct EndingMenuPlugin;

impl Plugin for EndingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::EndMenuWinP1)
            .with_system(setup_end_menu_p1))
            .add_system_set(SystemSet::on_update(AppState::EndMenuWinP1)
                .with_system(image_blinking)
                .with_system(back_to_game))
            .add_system_set(SystemSet::on_exit(AppState::EndMenuWinP1)
                .with_system(cleanup_menu))

            .add_system_set(SystemSet::on_enter(AppState::EndMenuWinP2)
                .with_system(setup_end_menu_p2))
            .add_system_set(SystemSet::on_update(AppState::EndMenuWinP2)
                .with_system(image_blinking)
                .with_system(back_to_game))
            .add_system_set(SystemSet::on_exit(AppState::EndMenuWinP2)
                .with_system(cleanup_menu))

            .add_system_set(SystemSet::on_enter(AppState::EndMenuDraw)
                .with_system(setup_end_menu_draw))
            .add_system_set(SystemSet::on_update(AppState::EndMenuDraw)
                .with_system(image_blinking)
                .with_system(back_to_game))
            .add_system_set(SystemSet::on_exit(AppState::EndMenuDraw)
                .with_system(cleanup_menu));
    }
}

pub fn setup_end_menu_p1(mut commands: Commands, textures: Res<MenuTextures>) {
    spawn_main_image(&mut commands, textures.win1_1.clone(), false,
                     SLOW_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);

    spawn_main_image(&mut commands, textures.win1_2.clone(), true,
                     SLOW_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);

    spawn_main_image(&mut commands, textures.play_again.clone(), true,
                     SUPER_SLOW_BLINKING, BELOW_IMAGE_SIZE, BELOW_IMAGE_POSITION);
}

pub fn setup_end_menu_p2(mut commands: Commands, textures: Res<MenuTextures>) {
    spawn_main_image(&mut commands, textures.win2_1.clone(), false,
                     SLOW_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);

    spawn_main_image(&mut commands, textures.win2_2.clone(), true,
                     SLOW_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);

    spawn_main_image(&mut commands, textures.play_again.clone(), true,
                     SUPER_SLOW_BLINKING, BELOW_IMAGE_SIZE, BELOW_IMAGE_POSITION);
}

pub fn setup_end_menu_draw(mut commands: Commands, textures: Res<MenuTextures>) {
    spawn_main_image(&mut commands, textures.draw1.clone(), false,
                     SLOW_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);

    spawn_main_image(&mut commands, textures.draw2.clone(), true,
                     SLOW_BLINKING, MAIN_IMAGE_SIZE, MAIN_IMAGE_POSITION);

    spawn_main_image(&mut commands, textures.play_again.clone(), true,
                     SUPER_SLOW_BLINKING, BELOW_IMAGE_SIZE, BELOW_IMAGE_POSITION);
}