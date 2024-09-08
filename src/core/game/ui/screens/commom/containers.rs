use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct UIContainerFlexCenter {
    node_bundle: NodeBundle,
    name: Name,
}

impl UIContainerFlexCenter {
    pub fn new(name: &str) -> Self {
        Self {
            node_bundle: NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    column_gap: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::from(BLACK)),
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            },
            name: Name::new(name.to_string()),
        }
    }
}
