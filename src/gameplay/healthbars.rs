use bevy::prelude::*;

#[derive(Component)]
pub struct Healthbars;


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
//         image: asset_server.load("images/menu/press_key2.png").into(),
//         ..default()
//     }).id();
//
//     commands.entity(starting_text).insert(MenuEntities {
//         menu_entity: starting_text,
//         time: BLINKING_TIME,
//     });
// }