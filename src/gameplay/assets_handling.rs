use bevy::prelude::*;

#[derive(Component)]
pub struct GameTextures {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,

    pub sword: Handle<Image>,
    pub sword2: Handle<Image>,

    pub healthbar1: Handle<Image>,
    pub healthbar2: Handle<Image>,
}

impl GameTextures {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        GameTextures {
            player_left: asset_server.load("images/samurai1.png"),
            player_right: asset_server.load("images/samurai2.png"),
            sword: asset_server.load("images/sword.png"),
            sword2: asset_server.load("images/sword2.png"),

            healthbar1: asset_server.load("images/healthbars/hearts_1_bundle.png"),
            healthbar2: asset_server.load("images/healthbars/hearts_2_bundle.png"),
        }
    }
}