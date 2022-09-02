use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy::render::settings::Backends;
use crate::CursorIcon::Default;
use crate::gameplay::{GameTextures, Player};

pub fn generate_sprite_sheet(
    image: Handle<Image>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tile_size: Vec2,
    grid: (usize, usize),
    size: Vec2,
    cords: Vec3,
) -> SpriteSheetBundle {
    let texture_handle = image;
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, grid.0, grid.1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_xyz(cords.x, cords.y, cords.z),
        sprite: TextureAtlasSprite {
            custom_size: Some(size),
            ..default()
        },
        ..default()
    }
}

pub fn spawn_animated_sprite(mut commands: Commands,
                             image: Handle<Image>,
                             mut texture_atlases:
                             ResMut<Assets<TextureAtlas>>,
                             tile_size: Vec2,
                             grid: (usize, usize),
                             size: Vec2,
                             cords: Vec3) {
    commands
        .spawn_bundle(generate_sprite_sheet(image, texture_atlases, tile_size, grid, size, cords))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn spawn_dragon(mut commands: Commands,
                    textures: Res<GameTextures>,
                    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_sprite(commands, textures.dragon.clone(), texture_atlases,
                          Vec2::new(500.0, 400.0),
                          (1, 1),
                          Vec2::new(200.0, 160.0),
                          Vec3::new(0.0, 280.0, 5.0),
    );
}

pub fn spawn_healthbar1(mut commands: Commands,
                        textures: Res<GameTextures>,
                        mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_sprite(commands, textures.healthbar1.clone(), texture_atlases,
                          Vec2::new(960.0, 168.0),
                          (1, 13),
                          Vec2::new(300.0, 52.5),
                          Vec3::new(-250.0, 300.0, 5.0),
    );
}

pub fn spawn_healthbar2(mut commands: Commands,
                        textures: Res<GameTextures>,
                        mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_sprite(commands, textures.healthbar2.clone(), texture_atlases,
                          Vec2::new(960.0, 168.0),
                          (1, 13),
                          Vec2::new(300.0, 52.5),
                          Vec3::new(250.0, 300.0, 5.0),
    );
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>)>,
    mut player_query: Query<(&mut Player)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());

        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        if sprite.index > 6 {
            if timer.just_finished() {
                sprite.index = (sprite.index + 6) % texture_atlas.textures.len();
            }
        } else {
            for player in player_query.borrow_mut() {
                if player.hurting > 0.0 {
                    match player.hp {
                        81.0..=100.0 => { sprite.index = 6 % texture_atlas.textures.len(); }
                        61.0..=80.0 => { sprite.index = 7 % texture_atlas.textures.len(); }
                        41.0..=60.0 => { sprite.index = 8 % texture_atlas.textures.len(); }
                        21.0..=40.0 => { sprite.index = 9 % texture_atlas.textures.len(); }
                        11.0..=20.0 => { sprite.index = 10 % texture_atlas.textures.len(); }
                        0.0..=10.0 => { sprite.index = 11 % texture_atlas.textures.len(); }
                        _ => { sprite.index = 12 % texture_atlas.textures.len(); }
                    }
                } else {
                    match player.hp {
                        81.0..=100.0 => { sprite.index = 0 % texture_atlas.textures.len(); }
                        61.0..=80.0 => { sprite.index = 1 % texture_atlas.textures.len(); }
                        41.0..=60.0 => { sprite.index = 2 % texture_atlas.textures.len(); }
                        21.0..=40.0 => { sprite.index = 3 % texture_atlas.textures.len(); }
                        11.0..=20.0 => { sprite.index = 4 % texture_atlas.textures.len(); }
                        0.0..=10.0 => { sprite.index = 5 % texture_atlas.textures.len(); }
                        _ => { sprite.index = 12 % texture_atlas.textures.len(); }
                    }
                }
            }
        }
    }
}

