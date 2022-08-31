use bevy::prelude::*;

mod controls;
mod assets_handling;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // app.
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuTextures::load(asset_server));
}



impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        //todo!()
    }
}