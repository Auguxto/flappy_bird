use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

use crate::core::overlay::OverlayPlugin;

pub mod window_systems;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::window::WindowPlugin {
            // Window Setup
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                name: Some("Flappy Bird".into()),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::BorderlessFullscreen,
                visible: false,
                ..default()
            }),
            ..default()
        })
        // Default window background color
        .insert_resource(ClearColor(Color::from(Srgba::hex("4ec0ca").unwrap())))
        // Overlays
        .add_plugins(OverlayPlugin)
        .add_systems(Update, window_systems::make_window_visible);
    }
}
