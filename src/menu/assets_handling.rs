use bevy::prelude::*;

#[derive(Component)]
pub struct MenuTextures {
    pub starting_text: Handle<Image>,

    pub win1_1: Handle<Image>,
    pub win1_2: Handle<Image>,

    pub win2_1: Handle<Image>,
    pub win2_2: Handle<Image>,

    pub draw1: Handle<Image>,
    pub draw2: Handle<Image>,
}

impl MenuTextures {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        MenuTextures {
            starting_text: asset_server.load("images/menu/press_key.png"),

            win1_1: asset_server.load("images/menu/win1_1.png"),
            win1_2: asset_server.load("images/menu/win1_2.png"),

            win2_1: asset_server.load("images/menu/win2_1.png"),
            win2_2: asset_server.load("images/menu/win2_2.png"),

            draw1: asset_server.load("images/menu/draw1.png"),
            draw2: asset_server.load("images/menu/draw2.png"),
        }
    }
}