use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use crate::core::game::state::state_states::ScreenState;

use fps_systems::*;

mod fps_systems;

pub struct FpsOverlayPlugin;

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(ScreenState::InGame), setup_fps_counter)
            .add_systems(OnEnter(ScreenState::MainMenu), despawn_fps_conuter)
            .add_systems(OnEnter(ScreenState::Dead), despawn_fps_conuter)
            .add_systems(
                Update,
                (fps_text_update_system, fps_counter_showhide)
                    .run_if(in_state(ScreenState::InGame)),
            );
    }
}
