use std::borrow::BorrowMut;
use bevy::prelude::*;
use crate::menu::MenuEntities;

pub fn image_blinking(mut menu_images: Query<(&mut MenuEntities, &mut Visibility)>) {
    for (mut image, mut visibility) in menu_images.borrow_mut() {
        if image.blinking {
            if image.current_time < 0.0 {
                visibility.is_visible = !visibility.is_visible;
                image.current_time = image.time;
            } else {
                image.current_time -= 1.0;
            }
        }
    }
}

pub fn spawn_main_image(commands: &mut Commands,
                        image: Handle<Image>,
                        blinking: bool,
                        blinking_time: f32,
                        pixels: (f32, f32),
                        position: (f32, f32))
{
    let main_image = commands.spawn_bundle(ImageBundle {
        style: Style {
            size: Size::new(Val::Px(pixels.0), Val::Px(pixels.1)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(position.0),
                bottom: Val::Px(position.1),
                ..default()
            },
            ..default()
        },
        image: UiImage::from(image),
        ..default()
    }).id();

    commands.entity(main_image).insert(MenuEntities {
        menu_entity: main_image,
        time: blinking_time,
        current_time: blinking_time,
        blinking,
    });
}