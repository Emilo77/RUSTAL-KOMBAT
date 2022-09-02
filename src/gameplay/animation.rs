use bevy::prelude::*;
use crate::gameplay::{PlayerNum, PlayerNumComponent};

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
                             texture_atlases:
                             ResMut<Assets<TextureAtlas>>,
                             tile_size: Vec2,
                             grid: (usize, usize),
                             size: Vec2,
                             cords: Vec3,
                             num: PlayerNum) {
    commands
        .spawn_bundle(generate_sprite_sheet(image, texture_atlases, tile_size, grid, size, cords))
        .insert(PlayerNumComponent::new(num));
}




