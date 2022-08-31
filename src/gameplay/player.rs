use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy::utils::tracing::instrument::WithSubscriber;
use bevy_rapier2d::parry::simba::simd::WideBoolF32x4;
use rand::prelude::ThreadRng;
use crate::AppState;

use crate::gameplay::{Abilities, Bounds, create_sprite_bundle, GameTextures, spawn_dynamic_object};

pub struct PlayerPlugin;

const LEFT_PLAYER_CORDS: (f32, f32, f32) = (-500.0, -200.0, 5.0);
const RIGHT_PLAYER_CORDS: (f32, f32, f32) = (500.0, -200.0, 5.0);
const PLAYER_BASIC_SPEED: f32 = 500.0;
const PLAYER_MAX_SPEED: f32 = 700.0;
const PLAYER_STARTING_HP: usize = 100;

#[derive(Eq, PartialEq)]
enum PlayerNum {
    One,
    Two,
}

enum PlayerSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    num: PlayerNum,
    hp: usize,
    speed: f32,
    max_speed: f32,
    side: PlayerSide,
}

impl Player {
    fn new(num: PlayerNum) -> Self {
        Player {
            hp: PLAYER_STARTING_HP,
            speed: PLAYER_BASIC_SPEED,
            max_speed: PLAYER_MAX_SPEED,
            side: match num {
                PlayerNum::One => { PlayerSide::Right }
                PlayerNum::Two => { PlayerSide::Left }
            },
            num,
        }
    }

    fn spawn_left(commands: &mut Commands, game_textures: Res<GameTextures>, cords: (f32, f32,
                                                                                     f32)) {
        let mut player_entity = spawn_dynamic_object(
            commands,
            create_sprite_bundle(game_textures.player_left.clone(),
                                 (300.0, 300.0), cords),
            None,
            None,
        );
        commands.entity(player_entity)
            .insert(Player::new(PlayerNum::One))
            .insert(Abilities::default());
    }

    fn spawn_right(commands: &mut Commands, game_textures: Res<GameTextures>,
                   cords: (f32, f32, f32)) {
        let mut player_entity = spawn_dynamic_object(
            commands,
            create_sprite_bundle(game_textures.player_right.clone(),
                                 (300.0, 300.0), cords),
            None,
            None,
        );
        commands.entity(player_entity)
            .insert(Player::new(PlayerNum::Two))
            .insert(Abilities::default());
    }
}

pub fn spawn_players(
    mut commands: Commands,
    game_textures_1: Res<GameTextures>,
    game_textures_2: Res<GameTextures>,
) {
    Player::spawn_left(&mut commands, game_textures_1, LEFT_PLAYER_CORDS);
    Player::spawn_right(&mut commands, game_textures_2, RIGHT_PLAYER_CORDS);
}

fn run_quicker(mut player: &mut Player) {
    if player.speed < player.max_speed {
        player.speed += 1.0;
    }
}

fn run_normal_speed(mut player: &mut Player) {
    player.speed = PLAYER_BASIC_SPEED;
}

fn players_movement(mut player_query: Query<(&mut Player, &mut Transform)>,
                    keyboard_input: Res<Input<KeyCode>>,
                    time: Res<Time>) {
    for (mut player, mut transform) in player_query.borrow_mut() {
        match player.num {
            PlayerNum::One => {
                if keyboard_input.pressed(KeyCode::A) {
                    transform.translation.x -= player.speed * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::D) {
                    transform.translation.x += player.speed * time.delta_seconds();
                }
                Bounds::check_bounds_x(&mut transform);
            }
            PlayerNum::Two => {
                if keyboard_input.pressed(KeyCode::Left) {
                    transform.translation.x -= player.speed * time.delta_seconds();
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    transform.translation.x += player.speed * time.delta_seconds();
                }
                Bounds::check_bounds_x(&mut transform);
            }
        }
    }
}

fn jumping(mut player_query: Query<(&mut Player, &mut Transform, &mut Abilities)>,
                  keyboard_input: Res<Input<KeyCode>>,
                  time: Res<Time>) {
    for (mut player, mut transform, mut abilities) in player_query.borrow_mut() {
        match player.num {
            PlayerNum::One => {
                Abilities::handle_all(&mut abilities, &mut transform);
                if keyboard_input.just_pressed(KeyCode::W) {
                    Abilities::handle_jump(&mut abilities, &mut transform);
                }
            }
            PlayerNum::Two => {
                Abilities::handle_all(&mut abilities, &mut transform);
                if keyboard_input.just_pressed(KeyCode::Up) {
                    Abilities::handle_jump(&mut abilities, &mut transform);
                }
            }
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame)
            .with_system(spawn_players))
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(players_movement)
                .with_system(jumping));

        // .add_system(camera_following_players);
    }
}


