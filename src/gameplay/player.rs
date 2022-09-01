use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy::utils::tracing::instrument::WithSubscriber;
use bevy_rapier2d::parry::simba::simd::WideBoolF32x4;
use rand::prelude::ThreadRng;
use crate::AppState;

use crate::gameplay::{Abilities, Bounds, cleanup_all, create_sprite_bundle, dash_system, GameTextures, jumping, movement, overall_combat, spawn_dynamic_object};

pub struct PlayerPlugin;

//CONTROLS
const CONTROLS_PLAYER1_LEFT : KeyCode = KeyCode::A;
const CONTROLS_PLAYER1_RIGHT : KeyCode = KeyCode::D;
const CONTROLS_PLAYER1_JUMP : KeyCode = KeyCode::W;
const CONTROLS_PLAYER1_DASH : KeyCode = KeyCode::S;
const CONTROLS_PLAYER1_ATTACK : KeyCode = KeyCode::Space;

const CONTROLS_PLAYER2_LEFT : KeyCode = KeyCode::Left;
const CONTROLS_PLAYER2_RIGHT : KeyCode = KeyCode::Right;
const CONTROLS_PLAYER2_JUMP : KeyCode = KeyCode::Up;
const CONTROLS_PLAYER2_DASH : KeyCode = KeyCode::Down;
const CONTROLS_PLAYER2_ATTACK : KeyCode = KeyCode::Numpad0;

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub dash: KeyCode,
    pub attack: KeyCode
}

const LEFT_PLAYER_CORDS: (f32, f32, f32) = (-500.0, -200.0, 5.0);
const RIGHT_PLAYER_CORDS: (f32, f32, f32) = (500.0, -200.0, 5.0);
const PLAYER_BASIC_SPEED: f32 = 5.0;
const PLAYER_MAX_SPEED: f32 = 7.0;
const PLAYER_STARTING_HP: usize = 100;



#[derive(Eq, PartialEq)]
pub enum PlayerNum {
    One,
    Two,
}

#[derive(Copy, Clone)]
pub enum PlayerSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub num: PlayerNum,
    pub hp: usize,
    pub speed: f32,
    pub max_speed: f32,
    pub side: PlayerSide,
    pub controls: Controls,
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
            controls: match num {
                PlayerNum::One => {
                    Controls {
                        left: CONTROLS_PLAYER1_LEFT,
                        right: CONTROLS_PLAYER1_RIGHT,
                        jump: CONTROLS_PLAYER1_JUMP,
                        dash: CONTROLS_PLAYER1_DASH,
                        attack: CONTROLS_PLAYER1_ATTACK,
                    }
                }
                PlayerNum::Two => {
                    Controls {
                        left: CONTROLS_PLAYER2_LEFT,
                        right: CONTROLS_PLAYER2_RIGHT,
                        jump: CONTROLS_PLAYER2_JUMP,
                        dash: CONTROLS_PLAYER2_DASH,
                        attack: CONTROLS_PLAYER2_ATTACK,
                    }
                }
            },
            num,

        }
    }

    fn spawn_left(commands: &mut Commands, game_textures: Res<GameTextures>, cords: (f32, f32,
                                                                                     f32)) {
        let mut player_entity = spawn_dynamic_object(
            commands,
            create_sprite_bundle(game_textures.player_left.clone(),
                                 (300.0, 150.0), cords),
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


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame)
            .with_system(spawn_players))
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(movement)
                .with_system(jumping)
                .with_system(dash_system)
                .with_system(overall_combat));
    }
}


