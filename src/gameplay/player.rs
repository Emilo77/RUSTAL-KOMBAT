use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy::utils::tracing::instrument::WithSubscriber;
use bevy_rapier2d::parry::simba::simd::WideBoolF32x4;
use rand::prelude::ThreadRng;
use crate::AppState;

use crate::gameplay::{Abilities, Bounds, GameTextures,
                      cleanup_all, create_sprite_bundle,
                      dash_system, jumping, kill, movement,
                      overall_combat, spawn_dynamic_object};
use crate::gameplay::animation::*;
use crate::gameplay::boards::{animate_healthbars, spawn_dragon, spawn_healthbar1, spawn_healthbar2};

pub struct PlayerPlugin;

//CONTROLS
const CONTROLS_PLAYER1_LEFT: KeyCode = KeyCode::A;
const CONTROLS_PLAYER1_RIGHT: KeyCode = KeyCode::D;
const CONTROLS_PLAYER1_JUMP: KeyCode = KeyCode::W;
const CONTROLS_PLAYER1_DASH: KeyCode = KeyCode::S;
const CONTROLS_PLAYER1_ATTACK: KeyCode = KeyCode::Space;

const CONTROLS_PLAYER2_LEFT: KeyCode = KeyCode::Left;
const CONTROLS_PLAYER2_RIGHT: KeyCode = KeyCode::Right;
const CONTROLS_PLAYER2_JUMP: KeyCode = KeyCode::Up;
const CONTROLS_PLAYER2_DASH: KeyCode = KeyCode::Down;
const CONTROLS_PLAYER2_ATTACK: KeyCode = KeyCode::Numpad0;

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub dash: KeyCode,
    pub attack: KeyCode,
}

const LEFT_PLAYER_CORDS: (f32, f32, f32) = (-500.0, -200.0, 5.0);
const RIGHT_PLAYER_CORDS: (f32, f32, f32) = (500.0, -200.0, 5.0);
const PLAYER_BASIC_SPEED: f32 = 5.0;
const PLAYER_STARTING_HP: f32 = 100.0;


#[derive(Component)]
pub struct PlayerNumComponent {
    pub num: PlayerNum,
}

impl PlayerNumComponent {
    pub fn new(num: PlayerNum) -> Self {
        PlayerNumComponent {
            num
        }
    }
}

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
    pub hp: f32,
    pub speed: f32,
    pub side: PlayerSide,
    pub controls: Controls,
    pub hurting: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame)
            .with_system(spawn_players)
            .with_system(spawn_dragon)
            .with_system(spawn_healthbar1)
            .with_system(spawn_healthbar2))
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(animate_healthbars)
                .with_system(movement)
                .with_system(jumping)
                .with_system(dash_system)
                .with_system(overall_combat)
                .with_system(kill)
                .with_system(handle_death));
    }
}

impl Player {
    fn new(num: PlayerNum) -> Self {
        Player {
            hp: PLAYER_STARTING_HP,
            speed: PLAYER_BASIC_SPEED,
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
            hurting: 0.0,
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

fn handle_death(mut players: Query<(&mut Player)>, mut app_state: ResMut<State<AppState>>) {
    let mut players_killed: (bool, bool) = (false, false);

    for player in players.borrow_mut() {
        if player.hp <= 0.0 {
            match player.num {
                PlayerNum::One => { players_killed.0 = true; }
                PlayerNum::Two => { players_killed.1 = true; }
            }
        }
    }

    match players_killed {
        (true, true) => { app_state.set(AppState::EndMenuDraw).unwrap(); }
        (false, true) => { app_state.set(AppState::EndMenuWinP1).unwrap(); }
        (true, false) => { app_state.set(AppState::EndMenuWinP2).unwrap(); }
        _ => {}
    }
}





