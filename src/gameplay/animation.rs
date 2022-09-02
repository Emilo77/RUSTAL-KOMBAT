use bevy::prelude::*;
use crate::gameplay::{generate_sprite_sheet, PlayerNum, PlayerNumComponent};


pub fn spawn_animated_object(mut commands: Commands,
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




