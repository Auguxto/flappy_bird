use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod fps_systems;

pub struct FpsOverlayPlugin;

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, fps_systems::setup_fps_counter)
            .add_systems(
                Update,
                (
                    fps_systems::fps_text_update_system,
                    fps_systems::fps_counter_showhide,
                ),
            );
    }
}
