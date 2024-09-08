use bevy::{
    color::palettes::css,
    prelude::{TextBundle, *},
};

use crate::config::FONT_FAMILY;

#[derive(Bundle)]
pub struct UIText {
    text: TextBundle,
}

impl UIText {
    pub fn new(value: &str, asset_server: &mut ResMut<AssetServer>) -> Self {
        Self {
            text: TextBundle {
                text: Text::from_section(
                    value,
                    TextStyle {
                        font_size: 32.0,
                        color: Color::from(css::WHITE),
                        font: asset_server.load(FONT_FAMILY),
                    },
                ),
                ..default()
            },
        }
    }
}
