use bevy::prelude::*;

#[derive(Component)]
pub struct Healthbar {
    entity: Entity,
}

// fn generate_healthbars(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let healthbar1 = commands.spawn_bundle(ImageBundle {
//         style: Style {
//             size: Size::new(Val::Px(1000.0), Val::Px(600.0)),
//             position: UiRect {
//                 left: Val::Px(150.0),
//                 bottom: Val::Px(100.0),
//                 ..default()
//             },
//             ..default()
//         },
//         image: asset_server.load("images/menu/press_key.png").into(),
//         ..default()
//     }).id();
//
//     commands.entity(healthbar1).insert(Healthbar {
//         menu_entity: starting_text,
//         time: BLINKING_TIME,
//     });
// }
//
// pub fn spawn_bird(
//     commands: &mut Commands,
//     asset_server: &mut Res<AssetServer>,
//     mut textures: &mut ResMut<Assets<Texture>>,
//     texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture_handle = asset_server
//         .load_sync(&mut textures, "assets/bird.png")
//         .unwrap();
//
//     let texture = textures.get(&texture_handle).unwrap();
//     let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 2);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//
//     commands
//         .spawn(SpriteSheetComponents {
//             texture_atlas: texture_atlas_handle,
//             scale: Scale(6.0),
//             translation: Translation::new(0.0, 0.0, 100.0),
//             draw: Draw {
//                 is_transparent: true,
//                 is_visible: true,
//                 render_commands: Vec::new(),
//             },
//             ..Default::default()
//         })
//         .with(Timer::from_seconds(0.1, true))
//         .with(Player)
//         .with(AffectedByGravity)
//         .with(VelocityRotator {
//             angle_up: std::f32::consts::PI * 0.5 * 0.7,
//             angle_down: -std::f32::consts::PI * 0.5 * 0.5,
//             velocity_max: 400.0,
//         })
//         .with(Velocity(Vec2::zero()))
//         .with(Animations {
//             animations: vec![
//                 Animation {
//                     current_frame: 0,
//                     frames: vec![
//                         AnimationFrame {
//                             index: 0,
//                             time: 0.1,
//                         },
//                         AnimationFrame {
//                             index: 1,
//                             time: 0.1,
//                         },
//                         AnimationFrame {
//                             index: 2,
//                             time: 0.3,
//                         },
//                         AnimationFrame {
//                             index: 1,
//                             time: 0.1,
//                         },
//                     ],
//                 },
//                 Animation {
//                     current_frame: 0,
//                     frames: vec![AnimationFrame {
//                         index: 3,
//                         time: 0.2,
//                     }],
//                 },
//             ],
//             current_animation: 0,
//         });
// }

// fn setup_healthbars(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let starting_text = commands.spawn_bundle(ImageBundle {
//         style: Style {
//             size: Size::new(Val::Px(1000.0), Val::Px(600.0)),
//             position: UiRect {
//                 left: Val::Px(150.0),
//                 bottom: Val::Px(100.0),
//                 ..default()
//             },
//             ..default()
//         },
//         image: asset_server.load("images/menu/press_key.png").into(),
//         ..default()
//     }).id();
//
//     commands.entity(starting_text).insert(MenuEntities {
//         menu_entity: starting_text,
//         time: BLINKING_TIME,
//     });
// }