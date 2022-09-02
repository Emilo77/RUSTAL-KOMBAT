use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_parallax::{
    LayerData, ParallaxPlugin, ParallaxResource,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest())
            // Add parallax resource with layer data
            .insert_resource(ParallaxResource {
                layer_data: vec![
                    LayerData {
                        speed: 0.9,
                        path: "images/map/cyberpunk_back.png".to_string(),
                        tile_size: Vec2::new(96.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 0.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: 0.6,
                        path: "images/map/cyberpunk_middle.png".to_string(),
                        tile_size: Vec2::new(144.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 1.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: 0.1,
                        path: "images/map/cyberpunk_front.png".to_string(),
                        tile_size: Vec2::new(272.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 2.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: 1.0,
                        path: "images/map/cyberpunk_floor.png".to_string(),
                        tile_size: Vec2::new(272.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 3.0,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
            .add_plugin(ParallaxPlugin);
    }
}




