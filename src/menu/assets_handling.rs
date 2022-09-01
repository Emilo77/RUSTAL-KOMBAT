use bevy::prelude::*;

pub struct Textures {
    pub main: Handle<Image>,
    pub button: Handle<Image>,
    pub button_pressed: Handle<Image>
}

impl Textures {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        Textures {
            main: asset_server.load("images/menu.png"),
            button: asset_server.load("images/button.png"),
            button_pressed: asset_server.load("images/button_pressed.png"),
        }
    }
}

pub struct MenuMaterials {
    pub menu: UiColor,
    pub normal_button: UiColor,
    pub button_text: Color,
    pub button_hovered: UiColor,
    pub button_pressed: UiColor,
    pub button_selected: UiColor,
}

impl Default for MenuMaterials {
    fn default() -> Self {
        MenuMaterials {
            menu: Color::CRIMSON.into(),
            normal_button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_text: Color::rgb(0.9, 0.9, 0.9),
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            button_selected: Color::rgb(0.35, 0.35, 0.35).into(),
        }
    }
}