use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod systems;

pub struct FpsOverlayPlugin;

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, systems::setup_fps_counter)
            .add_systems(
                Update,
                (
                    systems::fps_text_update_system,
                    systems::fps_counter_showhide,
                ),
            );
    }
}
