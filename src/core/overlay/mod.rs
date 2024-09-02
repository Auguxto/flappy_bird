use bevy::prelude::*;

mod fps;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(fps::FpsOverlayPlugin);
    }
}
