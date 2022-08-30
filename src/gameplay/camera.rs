use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_parallax::{
    LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};

pub struct OwnCameraPlugin;

impl Plugin for OwnCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_camera_system)
            .add_system(move_camera_system);
    }
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(ParallaxCameraComponent);
}

pub fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: 0.5,
    });
}