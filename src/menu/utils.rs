// use bevy::prelude::*;
// use bevy::sprite::Rect;
//
// pub struct MenuData {
//     pub menu_entity: Entity,
//     pub camera_entity: Entity,
// }
//
// pub fn setup_main(
//     mut commands: Commands,
// ) {
//     let menu_entity = commands.spawn_bundle(menu_bundle).id();
//     let camera_entity = commands.spawn_bundle(UiCameraConfig::default()).id();
//     commands.insert_resource(MenuData {
//         menu_entity,
//         camera_entity,
//     });
// }
//
//
//
// fn menu_bundle() -> NodeBundle {
//     NodeBundle {
//         style: Style {
//             flex_direction: FlexDirection::ColumnReverse,
//             align_items: AlignItems::Center,
//             ..default()
//         },
//             ..default()
//     }
// }