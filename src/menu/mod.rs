use std::borrow::BorrowMut;
use bevy::prelude::*;

use crate::AppState;
use crate::CursorIcon::Default;
use crate::gameplay::{GameTextures};
use crate::menu::assets_handling::{MenuTextures};
use crate::menu::ending_menu::EndingMenuPlugin;
use crate::menu::starting_menu::StartingMenuPlugin;
use crate::menu::utils::image_blinking;

mod assets_handling;
mod utils;
mod starting_menu;
mod ending_menu;

pub const FAST_BLINKING: f32 = 50.0;
pub const SLOW_BLINKING: f32 = 50.0;
pub const SUPER_SLOW_BLINKING: f32 = 150.0;

pub const MAIN_IMAGE_POSITION: (f32, f32) = (250.0, 150.0);
pub const MAIN_IMAGE_SIZE: (f32, f32) = (750.0, 450.0);

pub const BELOW_IMAGE_SIZE: (f32, f32) = (300.0, 97.07);
pub const BELOW_IMAGE_POSITION: (f32, f32) = (600.0, 30.0);

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuEntities {
    pub menu_entity: Entity,
    pub time: f32,
    pub current_time: f32,
    pub blinking: bool,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup)
            .add_plugin(StartingMenuPlugin)
            .add_plugin(EndingMenuPlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuTextures::load(asset_server));
}

fn back_to_game(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    match *app_state.current() {
        AppState::InGame => {}
        _ => {
            if keys.just_pressed(KeyCode::Space) {
                app_state.set(AppState::InGame).unwrap();
                keys.reset(KeyCode::Escape);
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, mut menu_query: Query<&mut MenuEntities>) {
    for mut image in menu_query.borrow_mut() {
        commands.entity(image.menu_entity).despawn_recursive();
    }
}

