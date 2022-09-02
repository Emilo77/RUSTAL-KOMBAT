use std::borrow::BorrowMut;
use bevy::prelude::*;
use crate::gameplay::animation::spawn_animated_object;
use crate::gameplay::{GameTextures, Player, PlayerNum, PlayerNumComponent};


pub fn spawn_dragon(commands: Commands,
                    textures: Res<GameTextures>,
                    texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_object(commands, textures.dragon.clone(), texture_atlases,
                          Vec2::new(500.0, 400.0),
                          (1, 1),
                          Vec2::new(200.0, 160.0),
                          Vec3::new(0.0, 280.0, 5.0),
                          PlayerNum::One,
    );
}

pub fn spawn_healthbar1(commands: Commands,
                        textures: Res<GameTextures>,
                        texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_object(commands, textures.healthbar1.clone(), texture_atlases,
                          Vec2::new(960.0, 168.0),
                          (1, 13),
                          Vec2::new(300.0, 52.5),
                          Vec3::new(-250.0, 300.0, 5.0),
                          PlayerNum::One,
    );
}

pub fn spawn_healthbar2(commands: Commands,
                        textures: Res<GameTextures>,
                        texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_object(commands, textures.healthbar2.clone(), texture_atlases,
                          Vec2::new(960.0, 168.0),
                          (1, 13),
                          Vec2::new(300.0, 52.5),
                          Vec3::new(250.0, 300.0, 5.0),
                          PlayerNum::Two,
    );
}

pub fn animate_healthbars(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &PlayerNumComponent)>,
    mut player_query: Query<&mut Player>,
) {
    for (mut sprite, texture_atlas_handle, healthbar_num) in &mut query {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        for player in player_query.borrow_mut() {
            if healthbar_num.num == player.num {
                if player.hurting > 0.0 {
                    match player.hp {
                        81..=100 => { sprite.index = 6 % texture_atlas.textures.len(); }
                        61..=80 => { sprite.index = 7 % texture_atlas.textures.len(); }
                        41..=60 => { sprite.index = 8 % texture_atlas.textures.len(); }
                        21..=40 => { sprite.index = 9 % texture_atlas.textures.len(); }
                        11..=20 => { sprite.index = 10 % texture_atlas.textures.len(); }
                        0..=10 => { sprite.index = 11 % texture_atlas.textures.len(); }
                        _ => { sprite.index = 12 % texture_atlas.textures.len(); }
                    }
                } else {
                    match player.hp {
                        81..=100 => { sprite.index = 0 % texture_atlas.textures.len(); }
                        61..=80 => { sprite.index = 1 % texture_atlas.textures.len(); }
                        41..=60 => { sprite.index = 2 % texture_atlas.textures.len(); }
                        21..=40 => { sprite.index = 3 % texture_atlas.textures.len(); }
                        11..=20 => { sprite.index = 4 % texture_atlas.textures.len(); }
                        0..=10 => { sprite.index = 5 % texture_atlas.textures.len(); }
                        _ => { sprite.index = 12 % texture_atlas.textures.len(); }
                    }
                }
            }
        }
    }
}