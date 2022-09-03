use std::borrow::BorrowMut;
use bevy::prelude::*;
use crate::AppState;

use crate::gameplay::{Abilities, GameTextures, dashing, jumping, movement, overall_combat, spawn_dynamic_object, generate_sprite_sheet};
use crate::gameplay::boards::{animate_healthbars, spawn_dragon,
                              spawn_healthbar1, spawn_healthbar2};

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

//SPRITE SHEETS
const PLAYER_TILE_SIZE: Vec2 = Vec2::new(520.0, 1152.0);
const PLAYER_GRID: (usize, usize) = (6, 1);
pub const PLAYER_SIZE: Vec2 = Vec2::new(100.0, 221.54);

//STARTING CORDS
const LEFT_PLAYER_CORDS: (f32, f32, f32) = (-500.0, -200.0, 5.0);
const RIGHT_PLAYER_CORDS: (f32, f32, f32) = (500.0, -200.0, 5.0);

//POWER
const PLAYER_BASIC_SPEED: f32 = 5.0;
const PLAYER_STARTING_HP: i32 = 100;

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub dash: KeyCode,
    pub attack: KeyCode,
}

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

#[derive(Eq, PartialEq, Clone)]
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
    pub hp: i32,
    pub speed: f32,
    pub side: PlayerSide,
    pub controls: Controls,
    pub hurting: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame)
            .with_system(spawn_player1)
            .with_system(spawn_player2)
            .with_system(spawn_dragon)
            .with_system(spawn_healthbar1)
            .with_system(spawn_healthbar2))
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(animate_healthbars)
                .with_system(movement)
                .with_system(jumping)
                .with_system(dashing)
                .with_system(overall_combat)
                .with_system(handle_death)
                .with_system(player_animation));
    }
}

pub fn spawn_player1(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    Player::spawn(&mut commands,
                  game_textures,
                  texture_atlases,
                  PlayerNum::One);
}

pub fn spawn_player2(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    Player::spawn(&mut commands,
                  game_textures,
                  texture_atlases,
                  PlayerNum::Two);
}

fn handle_death(mut players: Query<&mut Player>, mut app_state: ResMut<State<AppState>>) {
    let mut players_killed: (bool, bool) = (false, false);

    for player in players.borrow_mut() {
        if player.hp <= 0 {
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

    fn spawn(commands: &mut Commands,
             game_textures: Res<GameTextures>,
             texture_atlases: ResMut<Assets<TextureAtlas>>,
             player_num: PlayerNum) {
        let cords: Vec3;
        let mut textures: Handle<Image>;
        match player_num {
            PlayerNum::One => {
                cords = Vec3::from(LEFT_PLAYER_CORDS);
                textures = game_textures.player_left.clone();
            }
            PlayerNum::Two => {
                cords = Vec3::from(RIGHT_PLAYER_CORDS);
                textures = game_textures.player_right.clone();
            }
        }

        let player_entity = spawn_dynamic_object(
            commands,
            generate_sprite_sheet(
                textures,
                texture_atlases,
                PLAYER_TILE_SIZE,
                PLAYER_GRID,
                PLAYER_SIZE,
                cords),
            None,
            None,
        );

        commands.entity(player_entity)
            .insert(Player::new(player_num.clone()))
            .insert(Abilities::new(player_num));
    }
}

pub fn player_animation(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Abilities)>,
    mut player_query: Query<&mut Player>,
) {
    for player in player_query.borrow_mut() {
        for (mut sprite, texture_atlas_handle, abilities) in &mut query {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            if abilities.player_num == player.num {
                match player.side {
                    PlayerSide::Left => {
                        if abilities.dash.is_active {
                            sprite.index = 2 % texture_atlas.textures.len();
                        } else {
                            sprite.index = 0 % texture_atlas.textures.len();
                        }
                    }
                    PlayerSide::Right => {
                        if abilities.dash.is_active {
                            sprite.index = 5 % texture_atlas.textures.len();
                        } else {
                            sprite.index = 3 % texture_atlas.textures.len();
                        }
                    }
                }
            }
        }
    }
}





