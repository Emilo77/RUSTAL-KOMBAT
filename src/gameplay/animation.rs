use bevy::prelude::*;
use crate::CursorIcon::Default;
use crate::gameplay::GameTextures;


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn generate_sprite_sheet(
    image: Handle<Image>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    size: (f32, f32),
    cords: (f32, f32),
) -> SpriteSheetBundle {
    let texture_handle = image;
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1920.0, 336.0), 1, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_xyz(cords.0, cords.1, 5.0),
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::new(size.0, size.1)),
            ..default()
        },
        ..default()
    }
}

pub fn spawn_animated_sprite(mut commands: Commands,
                             image: Handle<Image>,
                             mut texture_atlases:
                             ResMut<Assets<TextureAtlas>>,
                             size: (f32, f32), cords: (f32, f32),
) {
    commands
        .spawn_bundle(generate_sprite_sheet(image, texture_atlases, size, cords))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn spawn_animated(mut commands: Commands,
                      textures: Res<GameTextures>,
                      mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    spawn_animated_sprite(commands, textures.healthbar1.clone(), texture_atlases,
                          (800.0, 140.0), (0.0, 0.0));
    spawn_animated_sprite(commands, textures.healthbar1.clone(), texture_atlases,
                          (800.0, 140.0), (0.0, 0.0));
}
