use bevy::prelude::*;

#[derive(Component)]
pub struct GameTextures {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,

    pub sword: Handle<Image>,
    pub sword2: Handle<Image>,
}

impl GameTextures {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        GameTextures {
            player_left: asset_server.load("images/samurai1.png"),
            player_right: asset_server.load("images/samurai2.png"),
            sword: asset_server.load("images/sword.png"),
            sword2: asset_server.load("images/sword2.png"),
        }
    }
}