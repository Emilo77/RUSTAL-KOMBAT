use std::borrow::BorrowMut;
use std::os::linux::raw::stat;
use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy_rapier2d::parry::query::time_of_impact;
use crate::AppState;
use crate::CursorIcon::Default;
use crate::menu::assets_handling::{MenuMaterials, Textures};

mod controls;
mod assets_handling;
mod utils;

const BLINKING_TIME: f32 = 20.0;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuEntities {
    pub menu_entity: Entity,
    pub time: f32,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMaterials>()
            .add_system_set(SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu)
                .with_system(cleanup_menu))
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(back_to_game)
                .with_system(text_blinking)
        );
    }
}

fn back_to_game(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::MainMenu && keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
        keys.reset(KeyCode::Escape);
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let starting_text = commands.spawn_bundle(ImageBundle {
        style: Style {
            size: Size::new(Val::Px(1000.0), Val::Px(600.0)),
            position: UiRect {
                left: Val::Px(150.0),
                bottom: Val::Px(100.0),
                ..default()
            },
            ..default()
        },
        image: asset_server.load("images/menu/press_key2.png").into(),
        ..default()
    }).id();

    commands.entity(starting_text).insert(MenuEntities {
        menu_entity: starting_text,
        time: BLINKING_TIME,
    });
}

pub fn text_blinking(mut menu_images: Query<(&mut MenuEntities, &mut Visibility)>) {
    for (mut image, mut visibility) in menu_images.borrow_mut() {
        if image.time < 0.0 {
            visibility.is_visible = !visibility.is_visible;
            image.time = BLINKING_TIME;
        } else {
            image.time -= 1.0;
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, mut menu_query: Query<&mut MenuEntities>) {
    for mut image in menu_query.borrow_mut() {
        commands.entity(image.menu_entity).despawn_recursive();
    }
}