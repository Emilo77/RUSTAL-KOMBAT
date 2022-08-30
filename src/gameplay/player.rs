use bevy::prelude::*;
use rand::prelude::ThreadRng;
use crate::AppState;

use crate::gameplay::{create_sprite_bundle, GameTextures, spawn_dynamic_object};

pub struct PlayerPlugin;

const LEFT_PLAYER_CORDS: (f32, f32, f32) = (100.0, -200.0, 5.0);
const RIGHT_PLAYER_CORDS: (f32, f32, f32) = (620.0, -200.0, 5.0);

enum PlayerSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    hp: u8,
    velocity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            hp: 100,
            velocity: 1.0,
        }
    }
}

impl Player {
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
            .insert(Player::default());
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
            .insert(Player::default());
    }
}

pub fn spawn_players(
    mut commands: Commands,
    game_textures_1: Res<GameTextures>,
    game_textures_2: Res<GameTextures>,
    entities: Query<(Entity)>,
) {
    Player::spawn_left(&mut commands, game_textures_1, LEFT_PLAYER_CORDS);
    Player::spawn_right(&mut commands, game_textures_2, RIGHT_PLAYER_CORDS);
    for entity in entities.iter() {
        commands.insert_resource(entity);
    }
}

fn player1_movement(player_query: Query<(&Player, &mut Transform)>,
                   keyboard_input: Res<Input<KeyCode>>) {
    for (player, mut transform) in player_query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x - 1000.0;
            print!("Left key pressed!");
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x + 1000.0;
            print!("Right key pressed!");
        }
    }
}

fn player2_movement(player_query: Query<(&Player, &mut Transform)>,
                    keyboard_input: Res<Input<KeyCode>>) {
    for (player, mut transform) in player_query.iter() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x - 1000.0;
            print!("Left key pressed!");
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x + 1000.0;
            print!("Right key pressed!");
        }
    }
}


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_players)
            .add_system(player1_movement)
            .add_system(player2_movement);
        // .add_system(camera_following_players);
    }
}


